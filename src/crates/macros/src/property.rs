use std::collections::HashMap;

use crate::propagate_err;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    braced, ext::IdentExt, parenthesized, parse::Parse, parse_macro_input, punctuated::Punctuated,
    Token, Visibility,
};

pub(super) fn make_derive(item: TokenStream) -> TokenStream {
    let structure = parse_macro_input!(item as Structure);
    let attribute_info = propagate_err!(AttributeInfo::analyze(&structure));

    let property_structure = structure.build_property_structure(&attribute_info);
    let property_impls = structure.build_property_impls(&attribute_info);

    quote! {
        #property_structure
        #property_impls
    }
    .into()
}

struct Structure {
    attributes: Vec<syn::Attribute>,
    visibility: Visibility,
    struct_token: Token![struct],
    name: syn::Ident,
    braces: syn::token::Brace,
    fields: Punctuated<syn::Field, Token![,]>,
}

impl Structure {
    fn build_property_structure(&self, attribute_info: &AttributeInfo) -> proc_macro2::TokenStream {
        let derive_info = attribute_info
            .struct_attribute_info
            .derive
            .as_ref()
            .map(|derive| quote! { #derive })
            .unwrap_or_default();

        let ident = &attribute_info.struct_attribute_info.name;
        let Structure {
            ref visibility,
            ref struct_token,
            ref braces,
            ref fields,
            ..
        } = self;

        let mut tokens = quote! {
            #derive_info
            #visibility #struct_token #ident
        };

        braces.surround(&mut tokens, |tokens| {
            let wrapped_fields: Punctuated<syn::Field, Token![,]> = fields
                .clone()
                .into_iter()
                .map(|mut field| {
                    field.attrs.clear();
                    let field_name = field_name(&field);
                    let mut field_type = field.ty;

                    if let Some(field_attribute) =
                        attribute_info.field_attribute_infos.get(&field_name)
                    {
                        if let Some(use_type) = &field_attribute.use_type {
                            field_type = use_type.clone();
                        }
                    }
                    field.ty = wrap_by_option(field_type);
                    field
                })
                .collect();

            wrapped_fields.to_tokens(tokens);
        });

        tokens
    }

    fn build_property_impls(&self, attribute_info: &AttributeInfo) -> proc_macro2::TokenStream {
        let property_struct_name = &attribute_info.struct_attribute_info.name;
        let origin_struct_name = &self.name;

        let fn_merge = self.build_fn_merge(attribute_info);
        let fn_verify = self.build_fn_verify(attribute_info);
        let fn_unwrap_or_default = self.build_fn_unwrap_or_default(attribute_info);

        quote! {
            impl #property_struct_name {
                #fn_merge
                #fn_verify
                #fn_unwrap_or_default
            }

            impl From<#property_struct_name> for #origin_struct_name {
                fn from(val: #property_struct_name) -> Self {
                    val.unwrap_or_default()
                }
            }
        }
    }

    fn build_fn_merge(&self, attribute_info: &AttributeInfo) -> proc_macro2::TokenStream {
        let fields: Punctuated<&syn::Ident, Token![,]> = self
            .fields
            .iter()
            .map(|field| field.ident.as_ref().expect("Fields should be named"))
            .collect();

        let init_members: Punctuated<proc_macro2::TokenStream, Token![,]> = self
            .fields
            .iter()
            .map(|field| {
                let field_ident = field.ident.as_ref().expect("Fields should be named");
                let field_name = field_name(field);
                let mut line = quote! { #field_ident: #field_ident };

                if attribute_info
                    .field_attribute_infos
                    .get(&field_name)
                    .is_some_and(|field_info| field_info.mergeable)
                {
                    line = quote! { #line.map(|val| val.merge(other.#field_ident.clone())) }
                }

                quote! { #line.or(other.#field_ident) }
            })
            .collect();

        quote! {
            fn merge(self, other: Option<Self>) -> Self {
                let Some(other) = other else {
                    return self;
                };

                let Self { #fields } = self;
                Self {
                    #init_members
                }
            }
        }
    }

    fn build_fn_verify(&self, attribute_info: &AttributeInfo) -> proc_macro2::TokenStream {
        let body: Vec<proc_macro2::TokenStream> = self
            .fields
            .iter()
            .filter_map(|field| {
                let field_ident = field.ident.as_ref().expect("Fields should be named");
                let field_name = field_name(field);
                let field_info = attribute_info.field_attribute_infos.get(&field_name)?;

                Some(match field_info.verifier.as_ref()? {
                    Verifier::Composite { force_check } => {
                        let mut statement = quote! {
                            if let Some(field) = self.#field_ident.as_ref() {
                                field.verify()?;
                            }
                        };

                        if *force_check {
                            let error_message =
                                format!("The '{field_name}' config value wasn't set!");

                            statement = quote! {
                                #statement else {
                                    return Err(#error_message)?;
                                }
                            }
                        }

                        statement
                    }
                    Verifier::FunctionCall(path) => {
                        quote! {
                            #path(self.#field_ident.as_ref())?;
                        }
                    }
                })
            })
            .collect();

        quote! {
            fn verify(&self) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
                #(#body)*

                Ok(())
            }
        }
    }

    fn build_fn_unwrap_or_default(
        &self,
        attribute_info: &AttributeInfo,
    ) -> proc_macro2::TokenStream {
        let origin_struct_name = &self.name;

        let init_members: Punctuated<proc_macro2::TokenStream, Token![,]> = self
            .fields
            .clone()
            .into_iter()
            .map(|field| {
                let field_ident = field.ident.expect("Fields should be named");
                let field_name = field_ident.to_string();

                let mut init_line = quote! { #field_ident: self.#field_ident };

                if let Some(field_attribute) = attribute_info.field_attribute_infos.get(&field_name)
                {
                    match &field_attribute.default {
                        DefaultAssignment::CallDefault => {
                            init_line = quote! { #init_line.unwrap_or_default() }
                        }
                        DefaultAssignment::CallFunction(function) => {
                            init_line = quote! { #init_line.unwrap_or_else(#function) }
                        }
                        DefaultAssignment::Expression(expression) => {
                            init_line = quote! { #init_line.unwrap_or_else(|| #expression) }
                        }
                        DefaultAssignment::None => (),
                    }

                    if field_attribute.use_type.is_some() {
                        init_line = quote! { #init_line.into() }
                    }
                }

                init_line
            })
            .collect();

        quote! {
            fn unwrap_or_default(self) -> #origin_struct_name {
                #origin_struct_name {
                    #init_members
                }
            }
        }
    }
}

impl Parse for Structure {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            attributes: input.call(syn::Attribute::parse_outer)?,
            visibility: input.parse()?,
            struct_token: input.parse()?,
            name: input.parse()?,
            braces: braced!(content in input),
            fields: content.parse_terminated(syn::Field::parse_named, Token![,])?,
        })
    }
}

struct AttributeInfo {
    struct_attribute_info: StructAttributeInfo,
    field_attribute_infos: HashMap<String, FieldAttributeInfo>,
}

impl AttributeInfo {
    fn analyze(structure: &Structure) -> syn::Result<Self> {
        let struct_attribute_info: StructAttributeInfo = structure
            .attributes
            .clone()
            .into_iter()
            .find(Self::is_property)
            .ok_or(syn::Error::new(
                proc_macro2::Span::call_site(),
                "Expected #[property(name(StructName))] before struct declaration",
            ))
            .and_then(Self::attribute_inner_tokens)
            .and_then(syn::parse2)?;

        let mut field_attribute_infos = HashMap::new();
        for field in structure.fields.clone() {
            let field_name = field_name(&field);

            let Some(attribute) = field.attrs.clone().into_iter().find(Self::is_property) else {
                continue;
            };

            field_attribute_infos.insert(
                field_name,
                Self::attribute_inner_tokens(attribute).and_then(syn::parse2)?,
            );
        }

        Ok(Self {
            struct_attribute_info,
            field_attribute_infos,
        })
    }

    fn attribute_inner_tokens(attribute: syn::Attribute) -> syn::Result<proc_macro2::TokenStream> {
        if let syn::Meta::List(meta_list) = attribute.meta {
            Ok(meta_list.tokens)
        } else {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "Expected attribute like #[property()]",
            ))
        }
    }

    fn is_property(attribute: &syn::Attribute) -> bool {
        if let syn::Meta::List(meta_list) = &attribute.meta {
            meta_list.path.is_ident("property")
        } else {
            false
        }
    }
}

struct StructAttributeInfo {
    name: syn::Ident,
    derive: Option<DeriveInfo>,
}

impl Parse for StructAttributeInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let beginning_span = input.span();
        let mut name = None;
        let mut derive = None;

        loop {
            let ident: syn::Ident = input.parse()?;

            match &*ident.to_string() {
                "name" => {
                    let content;
                    let _paren = parenthesized!(content in input);
                    name = Some(content.parse()?)
                }
                "derive" => {
                    let content;
                    derive = Some(DeriveInfo {
                        ident,
                        paren: parenthesized!(content in input),
                        traits: content.parse_terminated(syn::Path::parse, Token![,])?,
                    })
                }
                _ => {
                    return Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        "Unknown attribute",
                    ))
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }

        let Some(name) = name else {
            return Err(syn::Error::new(
                beginning_span,
                "Expected \"name\" property for creating new struct",
            ));
        };

        Ok(Self { name, derive })
    }
}

struct DeriveInfo {
    ident: syn::Ident,
    paren: syn::token::Paren,
    traits: Punctuated<syn::Path, Token![,]>,
}

impl ToTokens for DeriveInfo {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        Token![#](self.ident.span()).to_tokens(tokens);
        syn::token::Bracket(self.ident.span()).surround(tokens, |tokens| {
            self.ident.to_tokens(tokens);
            self.paren.surround(tokens, |tokens| {
                self.traits.to_tokens(tokens);
            });
        });
    }
}

struct FieldAttributeInfo {
    mergeable: bool,
    default: DefaultAssignment,
    use_type: Option<syn::Type>,
    verifier: Option<Verifier>,
}

impl Parse for FieldAttributeInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut default = DefaultAssignment::None;
        let mut use_type = None;
        let mut mergeable = false;
        let mut verifier = None;

        loop {
            let ident: syn::Ident = input.parse()?;

            match &*ident.to_string() {
                "default" => {
                    default = if input.peek(syn::token::Paren) {
                        let content;
                        let _paren = parenthesized!(content in input);
                        content.parse()?
                    } else {
                        DefaultAssignment::CallDefault
                    }
                }
                "verifier" => {
                    let content;
                    let _paren = parenthesized!(content in input);
                    verifier = Some(content.parse()?);
                }
                "mergeable" => mergeable = true,
                "use_type" => {
                    let content;
                    let _paren = parenthesized!(content in input);
                    use_type = Some(content.parse()?)
                }
                _ => return Err(syn::Error::new(ident.span(), "Unknown attribute")),
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }

        Ok(Self {
            mergeable,
            default,
            use_type,
            verifier,
        })
    }
}

enum DefaultAssignment {
    CallDefault,
    CallFunction(syn::Path),
    Expression(syn::Expr),
    None,
}

impl Parse for DefaultAssignment {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(
            if input.peek(syn::Ident::peek_any) && input.peek2(Token![=]) {
                let ident: syn::Ident = input.parse()?;
                if ident != "path" {
                    return Err(syn::Error::new(proc_macro2::Span::call_site(), "Invalid declaration of default attribute. Correct usage: #[property(default(path = path::to::function))]"));
                }

                let _: Token![=] = input.parse()?;
                DefaultAssignment::CallFunction(input.parse()?)
            } else {
                DefaultAssignment::Expression(input.parse()?)
            },
        )
    }
}

fn wrap_by_option(ty: syn::Type) -> syn::Type {
    syn::Type::Path(syn::TypePath {
        qself: None,
        path: syn::Path {
            leading_colon: None,
            segments: Punctuated::from_iter(vec![
                syn::PathSegment {
                    ident: syn::Ident::new("std", proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::None,
                },
                syn::PathSegment {
                    ident: syn::Ident::new("option", proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::None,
                },
                syn::PathSegment {
                    ident: syn::Ident::new("Option", proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::AngleBracketed(
                        syn::AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: syn::token::Lt(proc_macro2::Span::call_site()),
                            args: Punctuated::from_iter(vec![syn::GenericArgument::Type(ty)]),
                            gt_token: syn::token::Gt(proc_macro2::Span::call_site()),
                        },
                    ),
                },
            ]),
        },
    })
}

enum Verifier {
    Composite { force_check: bool },
    FunctionCall(syn::Path),
}

impl Parse for Verifier {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path_ident: syn::Ident = input.parse()?;

        Ok(match &*path_ident.to_string() {
            "composite" => {
                if input.peek(Token![,]) {
                    let _comma: Token![,] = input.parse()?;
                    let force_check: syn::Ident = input.parse()?;
                    if force_check != "force_check" {
                        return Err(syn::Error::new(
                            force_check.span(),
                            format!("Expected 'force_check' but given {force_check}"),
                        ));
                    }

                    Verifier::Composite { force_check: true }
                } else {
                    Verifier::Composite { force_check: false }
                }
            }
            "path" => {
                let _eq_token = input.parse::<Token![=]>()?;

                Verifier::FunctionCall(input.parse()?)
            }
            _ => Err(syn::Error::new(
                path_ident.span(),
                format!("Expected 'composite' or 'path' arguments, but given {path_ident}"),
            ))?,
        })
    }
}

fn field_name(field: &syn::Field) -> String {
    field
        .ident
        .as_ref()
        .expect("Fields should be named")
        .to_string()
}
