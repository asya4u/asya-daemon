use std::{
    ffi::{c_char, CString},
    str::FromStr,
};

use super::FoundedPlugin;
use netcorehost::{
    hostfxr::ManagedFunction,
    nethost, pdcstring::{other::PdCStrExt, PdCStr, PdCString},
};
use plugin_api::ApiCallbacksMap;
use serde::de::Error;

pub struct DotnetRuntimePluginInfo {
    pub name: String,
    pub entry_point: ManagedFunction<extern "system" fn(*const c_char, *const ApiCallbacksMap)>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PreloadOptions {
    plugin_entry_point: String,
}

impl PreloadOptions {
    pub fn ep_namespace(&self) -> &str {
        self.plugin_entry_point.rsplitn(3, '.').last().unwrap()
    }

    pub fn ep_class(&self) -> &str {
        self.plugin_entry_point.rsplit('.').nth(1).unwrap()
    }

    pub fn ep_method(&self) -> &str {
        self.plugin_entry_point.rsplit('.').nth(0).unwrap()
    }
}

pub(crate) fn load_dotnet_plugin_data(
    libraries_path: &[FoundedPlugin],
) -> Vec<DotnetRuntimePluginInfo> {
    let mut res = Vec::with_capacity(libraries_path.len());
    for lib in libraries_path {
        if let FoundedPlugin::Dotnet {
            dll_path,
            runtimeconfig_path,
        } = lib
        {
            let config = dbg!(extract_preload_config(runtimeconfig_path).unwrap());

            let hostfxr = nethost::load_hostfxr().unwrap();
            let context = hostfxr
                .initialize_for_runtime_config(
                    PdCString::from_os_str(runtimeconfig_path.as_os_str()).unwrap(),
                )
                .unwrap();

            let delegate_loader = context
                .get_delegate_loader_for_assembly(
                    PdCString::from_os_str(dll_path.as_os_str()).unwrap(),
                )
                .unwrap();

            let entry_point = {
                let type_name = CString::from_str(
                    [
                        [config.ep_namespace(), config.ep_class()]
                            .join(".")
                            .as_str(),
                        config.ep_namespace(),
                    ]
                    .join(", ")
                    .as_str(),
                )
                .unwrap();

                delegate_loader
                .get_function_with_unmanaged_callers_only::<fn(config: *const c_char, callbacks: *const ApiCallbacksMap)>(
                    PdCStr::from_c_str(&type_name),
                    PdCStr::from_c_str(&CString::from_str(config.ep_method()).unwrap()),
                )
                .unwrap()
            };

            res.push(DotnetRuntimePluginInfo {
                name: dll_path.file_name().unwrap().to_str().unwrap().to_string(),
                entry_point,
            });
        }
    }
    res
}

fn extract_preload_config(
    runtimeconfig_path: &std::path::PathBuf,
) -> Result<PreloadOptions, serde_json::Error> {
    let opts_file_content =
        std::fs::read_to_string(runtimeconfig_path).map_err(serde_json::Error::io)?;

    let value: serde_json::Value = serde_json::from_str(&opts_file_content)?;
    let config_properties = value
        .pointer("/runtimeOptions/configProperties")
        .ok_or_else(|| {
            serde_json::Error::custom("Failed to find /runtimeOptions/configProperties in JSON")
        })?;

    serde_json::from_value(config_properties.clone())
}
