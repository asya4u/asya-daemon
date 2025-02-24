#[derive(macros::Property, Debug, PartialEq, Eq)]
#[property(name(ConfigProperty), derive(Debug, Clone))]
struct Config {
    #[property(default)]
    field1: i32,
    #[property(default)]
    field2: String,

    #[property(
        default(InnerConfigProperty { field1: Some(3), field2: Some(EnumType::Variant) }),
        use_type(InnerConfigProperty),
        mergeable,
        verifier(composite, force_check)
    )]
    field3: InnerConfig,
}

#[derive(macros::Property, PartialEq, Eq, Debug)]
#[property(name(InnerConfigProperty), derive(Debug, Clone))]
struct InnerConfig {
    #[property(default(4))]
    field1: u8,
    #[property(default, verifier(path = InnerConfig::check_enum_type))]
    field2: EnumType,
}

impl InnerConfig {
    fn check_enum_type(value: Option<&EnumType>) -> Result<(), Box<dyn std::error::Error>> {
        let Some(value) = value else {
            return Err("Value is empty")?;
        };

        match value {
            EnumType::Variant => Ok(()),
            EnumType::Invariant => Err("Invalid value!")?,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
enum EnumType {
    #[default]
    Variant,
    Invariant,
}

#[test]
fn create_and_compare() {
    let mut inner_config_property = InnerConfigProperty {
        field1: Some(3),
        field2: EnumType::Variant.into(),
    };

    assert_eq!(
        inner_config_property.clone().unwrap_or_default(),
        InnerConfig {
            field1: 3,
            field2: EnumType::Variant
        }
    );

    inner_config_property.field1 = None;

    assert_eq!(
        inner_config_property.clone().unwrap_or_default(),
        InnerConfig {
            field1: 4,
            field2: EnumType::Variant
        }
    );

    let mut config_property = ConfigProperty {
        field1: None,
        field2: Some("hell".to_string()),
        field3: None,
    };

    assert_eq!(
        config_property.clone().unwrap_or_default(),
        Config {
            field1: 0,
            field2: "hell".to_string(),
            field3: InnerConfig {
                field1: 3,
                field2: EnumType::Variant
            }
        }
    );

    inner_config_property.field2 = EnumType::Invariant.into();
    config_property.field3 = Some(inner_config_property.clone());
    assert_eq!(
        config_property.clone().unwrap_or_default(),
        Config {
            field1: 0,
            field2: "hell".to_string(),
            field3: InnerConfig {
                field1: 4,
                field2: EnumType::Invariant,
            },
        },
    );
}

#[test]
fn correct_unwrap_or_default() {
    let mut inner_config_property = InnerConfigProperty {
        field1: None,
        field2: None,
    };

    assert_eq!(
        inner_config_property.clone().unwrap_or_default(),
        InnerConfig {
            field1: 4,
            field2: EnumType::Variant
        }
    );

    inner_config_property.field1 = Some(2);
    assert_eq!(
        inner_config_property.clone().unwrap_or_default(),
        InnerConfig {
            field1: 2,
            field2: EnumType::Variant
        }
    );

    inner_config_property.field2 = EnumType::Invariant.into();
    assert_eq!(
        inner_config_property.clone().unwrap_or_default(),
        InnerConfig {
            field1: 2,
            field2: EnumType::Invariant
        }
    );

    inner_config_property.field1 = None;
    assert_eq!(
        inner_config_property.clone().unwrap_or_default(),
        InnerConfig {
            field1: 4,
            field2: EnumType::Invariant
        }
    );
}

#[test]
fn correct_merge() {
    let first_inner_config = InnerConfigProperty {
        field1: Some(8),
        field2: None,
    };

    let second_inner_config = InnerConfigProperty {
        field1: None,
        field2: Some(EnumType::Variant),
    };

    let merged = first_inner_config
        .clone()
        .merge(Some(second_inner_config.clone()));
    assert_eq!(
        merged.unwrap_or_default(),
        InnerConfig {
            field1: 8,
            field2: EnumType::Variant
        }
    );

    let first_config = ConfigProperty {
        field1: Some(10),
        field2: None,
        field3: Some(first_inner_config),
    };

    let second_config = ConfigProperty {
        field1: None,
        field2: Some("hell".to_string()),
        field3: Some(second_inner_config),
    };

    assert_eq!(
        first_config.merge(Some(second_config)).unwrap_or_default(),
        Config {
            field1: 10,
            field2: "hell".to_string(),
            field3: InnerConfig {
                field1: 8,
                field2: EnumType::Variant
            }
        }
    )
}

#[test]
fn check_verify_method() {
    let inner_config = InnerConfigProperty {
        field1: Some(8),
        field2: Some(EnumType::Variant),
    };

    assert!(inner_config.verify().is_ok());

    let mut config = ConfigProperty {
        field1: None,
        field2: None,
        field3: Some(inner_config),
    };

    assert!(config.verify().is_ok());

    config.field3.as_mut().unwrap().field2 = Some(EnumType::Invariant);

    assert!(config.verify().is_err());

    config.field3.as_mut().unwrap().field2 = None;

    assert!(config.verify().is_err());

    config.field3 = None;

    assert!(config.verify().is_err());
}
