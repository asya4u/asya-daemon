use derive_more::derive::{Deref, From, Into};
use serde::Serialize;
use std::{
    collections::HashMap,
    ffi::{CStr, CString, OsStr},
    fs, io,
    path::{Path, PathBuf},
    ptr::{self},
    thread,
};
use tokio::sync::{mpsc::Receiver, Mutex};
use tracing::*;

use shared::{
    configuration::{self, ConfigFieldType, CONFIG},
    event_system,
};

mod abstractions;
mod api_callbacks;

mod dotnet;
mod native;

#[derive(Debug, Serialize, Clone, Deref, From, Into)]
#[serde(into = "String")]
pub struct Sender(pub String);

#[derive(Debug, Serialize, Clone, Deref, From, Into)]
#[serde(into = "String")]
pub struct Event(pub String);

/// Event publishing from plugins.
#[derive(Debug, Serialize)]
pub struct PluginEvent {
    sender: Sender,
    data: Event,
}

#[derive(Debug, Clone)]
pub enum FoundedPlugin {
    Native {
        path: PathBuf,
    },
    Dotnet {
        dll_path: PathBuf,
        runtimeconfig_path: PathBuf,
    },
}

/// Loads plugins from path from config.
pub fn load_plugins(_receiver: Mutex<Receiver<String>>) {
    unsafe {
        let libraries_path = find_plugins();

        native::load_native_plugin_data(&libraries_path)
            .into_iter()
            .for_each(|native_plugin| {
                load_native(native_plugin);
            });

        dotnet::load_dotnet_plugin_data(&libraries_path)
            .into_iter()
            .for_each(|managed_plugin| {
                load_managed(managed_plugin);
            });
    };
}

unsafe fn load_managed(info: dotnet::DotnetRuntimePluginInfo) {
    let callback = info.entry_point;
    let name = info.name;
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let config = CONFIG
                .plugins
                .config
                .get_key_value(&name)
                .map(|(_, v)| crate::extract_config_ptr(v))
                .unwrap_or(ptr::null_mut())
                .cast_const();

            (callback)(config, Box::into_raw(Box::new(api_callbacks::get_api())));
        })
    });
}

unsafe fn load_native(el: native::NativePluginRuntimeInfo) {
    let callback = el.plugin_information.entrypoint;
    let name = CStr::from_ptr(el.plugin_information.name).to_str().unwrap();
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let _lib = el._library;
            let config = CONFIG
                .plugins
                .config
                .get_key_value(name)
                .map(|(_, v)| crate::extract_config_ptr(v))
                .unwrap_or(ptr::null_mut())
                .cast_const();

            (callback)(config, api_callbacks::get_api());
        })
    });
}

/// Finds plugins for user's OS and returns their paths.
fn find_plugins() -> Vec<FoundedPlugin> {
    let plugins_folder = &CONFIG.plugins.plugins_folder;
    let dir = Path::new(plugins_folder);

    let plugin_dirs = collect_plugin_dirs(dir).unwrap_or_default();
    let qualified_plugins = qualify_plugins(plugin_dirs);

    if qualified_plugins.is_empty() {
        info!(
            "No plugins found in folder '{}'.",
            dir.to_str().unwrap_or("ERROR DUE CASTING PLUGINS PATH")
        );
    } else {
        info!(
            "Found {} plugins: {:#?}",
            qualified_plugins.len(),
            qualified_plugins
        );
    }

    qualified_plugins
}

/// Collects all subdirectories in the plugins folder.
fn collect_plugin_dirs(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut plugin_dirs = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            plugin_dirs.push(path);
        }
    }

    Ok(plugin_dirs)
}

/// Qualifies plugins by scanning each plugin directory for .so or .dll+.json files.
fn qualify_plugins(plugin_dirs: Vec<PathBuf>) -> Vec<FoundedPlugin> {
    let mut res = vec![];

    for plugin_dir in plugin_dirs {
        // Collect all files in the plugin directory
        let files = collect_files(&plugin_dir).unwrap_or_default();

        // Check for .so files (Native plugins)
        for file in files
            .iter()
            .filter(|p| p.extension() == Some(OsStr::new("so")))
        {
            res.push(FoundedPlugin::Native { path: file.clone() });
        }

        // Check for .dll files and corresponding .json files (Dotnet plugins)
        for dll_file in files
            .iter()
            .filter(|p| p.extension() == Some(OsStr::new("dll")))
        {
            let filename = dll_file
                .file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.replace(".dll", ""))
                .unwrap_or_default();

            if let Some(json_file) = files.iter().find(|el| {
                let file_name = el.file_name().unwrap_or_default();
                let file_name_str = file_name.to_str().unwrap_or_default();
                file_name_str.starts_with(&filename) && file_name_str.ends_with("json")
            }) {
                res.push(FoundedPlugin::Dotnet {
                    dll_path: dll_file.clone(),
                    runtimeconfig_path: json_file.clone(),
                });
            }
        }
    }

    res
}

/// Collects all files in a given directory (not recursive).
fn collect_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files_with_extension = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files_with_extension.push(path);
        }
    }

    Ok(files_with_extension)
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadableRequest {
    pub request: String,
}

type ConfigEntry<'a> =
    &'a std::collections::HashMap<std::string::String, configuration::ConfigFieldType>;

fn extract_config_ptr(plugin_config: ConfigEntry) -> *mut i8 {
    let normalized_plugin_config = normalize_config(plugin_config);
    let stringified = serde_json::to_string(&normalized_plugin_config).unwrap();
    if let Ok(cstring) = CString::new(stringified.to_owned()) {
        CString::into_raw(cstring)
    } else {
        ptr::null_mut()
    }
}

fn normalize_config(
    plugin_config: &HashMap<String, configuration::ConfigFieldType>,
) -> HashMap<String, configuration::ConfigFieldType> {
    let mut res = HashMap::new();
    for (k, v) in plugin_config {
        let mut value_for_insert = v.to_owned();
        if let ConfigFieldType::Array(map) = v {
            let mut array_field = vec![String::new(); map.len()];
            for (i, element) in map {
                array_field[i - 1] = element.to_owned()
            }
            value_for_insert = ConfigFieldType::NormalizedArray(array_field);
        }
        res.insert(k.to_owned(), value_for_insert);
    }
    res
}
