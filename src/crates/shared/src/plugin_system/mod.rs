use libloading::Library;
use tracing::*;
use plugin_interface::{EventState, PluginInformation, State};
use serde::Serialize;
use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    fs, io,
    path::Path,
    ptr::{self},
    thread,
    time::Duration,
};
use tokio::sync::{mpsc::Receiver, Mutex};

use crate::{
    configuration::{self, ConfigFieldType, CONFIG},
    event_system,
};

mod api_callbacks;

// todo: редизайн типов чтобы такой хуеты как с Library не было
// !! порядок полей менять НЕЛЬЗЯ тоже может быть сегфолт
struct PluginRuntimeInfo {
    plugin_information: Box<PluginInformation>,
    _library: Library, // это поле вообще никгде не юзается, но без него сегфолт.
    state: *mut State,
}

/// Event publishing from plugins.
#[derive(Debug, Serialize)]
pub struct PluginEvent {
    sender: String,
    data: String,
}

/// Loads plugins from path from config.
pub fn load_plugins(receiver: Mutex<Receiver<String>>) {
    unsafe {
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let libraries_path = find_plugins();

                let mut plugins_data = load_plugin_data(libraries_path);

                do_loop(&mut plugins_data, receiver).await
            })
        })
    };
}

/// Finds plugins for user's OS and returs their pathes.
fn find_plugins() -> Vec<String> {
    let plugins_folder = &CONFIG.plugins.plugins_folder;
    let extension = if cfg!(target_family = "unix") {
        "so" // sal?
    } else {
        "dll"
    };

    let dir = Path::new(plugins_folder);
    let plugins = find_files_with_extension(dir, extension).unwrap_or_default();
    if plugins.is_empty() {
        info!(
            "No one plugins in folder '{}'.",
            dir.to_str().unwrap_or("ERROR DUE CASTING PLUGINS PATH")
        );
    } else {
        info!("Found {} plugins: {:?}", plugins.len(), plugins);
    }
    plugins
}

fn find_files_with_extension(dir: &Path, extension: &str) -> io::Result<Vec<String>> {
    let mut files_with_extension = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            if let Some(ext) = path.extension() {
                if ext == extension {
                    if let Some(path_str) = path.to_str() {
                        files_with_extension.push(path_str.to_string());
                    }
                }
            }
        }
    }

    Ok(files_with_extension)
}

async unsafe fn do_loop(plugins_data: &mut [PluginRuntimeInfo], receiver: Mutex<Receiver<String>>) {
    let mut recv = receiver.lock().await;
    loop {
        for info in &mut *plugins_data {
            if !recv.is_empty() {
                check_event_for_send(info, &mut recv).await;
            } else {
                (info.plugin_information.execute_callback)(info.state, api_callbacks::get_api());
            }
            check_event_for_publish(info).await;
        }
        tokio::time::sleep(Duration::from_micros(100)).await;
    }
}

async unsafe fn check_event_for_send<'a>(
    info: &mut PluginRuntimeInfo,
    event_recv: &mut tokio::sync::MutexGuard<'a, Receiver<String>>,
) {
    let event_callback = info.plugin_information.event_callback;
    let recieved_event = event_recv.recv().await;

    // Maybe we should pass some set of events instead one?
    let ptr = extract_ptr(recieved_event);

    let event_state = Box::into_raw(Box::new(EventState {
        state: info.state,
        event: ptr,
    }));
    (event_callback)(event_state, api_callbacks::get_api());
}

fn extract_ptr(res: Option<String>) -> *const std::ffi::c_char {
    CString::new(res.expect("mpsc for events was closed. this is a bug."))
        // release ownership here because memory frees in free_event_memory()
        .map(|cstring| cstring.into_raw().cast_const())
        .unwrap_or_else(|_| {
            warn!("Event string representation contains zero byte, which is not allowed.");
            ptr::null()
        })
}

async unsafe fn check_event_for_publish(info: &mut PluginRuntimeInfo) {
    if let Some(plugin_state) = ptr::NonNull::new(info.state) {
        check_event(plugin_state, info).await;
        check_request(plugin_state).await;

        free_memory(info);
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ReadableRequest(pub String);

async unsafe fn check_request(plugin_state: ptr::NonNull<State>) {
    if let Some(request_ptr) = ptr::NonNull::new(plugin_state.read().human_request) {
        let request_data = CStr::from_ptr(request_ptr.as_ptr()).to_str();
        if let Ok(str_data) = request_data {
            event_system::publish(ReadableRequest(str_data.to_string())).await;
        }
    }
}

async unsafe fn check_event(plugin_state: ptr::NonNull<State>, info: &mut PluginRuntimeInfo) {
    if let Some(published_event) = ptr::NonNull::new(plugin_state.read().published_event) {
        let published_event_data = CStr::from_ptr(published_event.as_ptr()).to_str();
        match published_event_data {
            Ok(str_data) => {
                let general_event = PluginEvent {
                    sender: CStr::from_ptr(info.plugin_information.name)
                        .to_str()
                        .unwrap() /* i think, plugin name will be not changed due asya
                        lifetime, so that we don't have to check this every time. */
                        .to_string(),
                    data: str_data.to_string(),
                };
                event_system::publish(general_event).await;
            }
            Err(_) => {
                warn!("Plugin sent an event, that cannot be represent as a valid Utf8 string. Event wasn'n published.")
            }
        }
    }
}

unsafe fn free_memory(info: &mut PluginRuntimeInfo) {
    let event_raw = (*info.state).published_event;
    if !event_raw.is_null() {
        drop(Box::from_raw(event_raw)); // panics if plugins frees memory independently, e.g. if after
                                        // casting event to CString
        (*info.state).published_event = ptr::null_mut()
    }

    let message_raw = (*info.state).readable_message;
    if !message_raw.is_null() {
        drop(Box::from_raw(message_raw)); // same as above
        (*info.state).readable_message = ptr::null_mut()
    }

    let request_raw = (*info.state).human_request;
    if !message_raw.is_null() {
        drop(Box::from_raw(request_raw)); // same as above
        (*info.state).human_request = ptr::null_mut()
    }
}

unsafe fn load_plugin_data(libs: Vec<String>) -> Vec<PluginRuntimeInfo> {
    const FN_PLUGIN_INFO: &[u8; 11] = b"plugin_info";
    let mut infos = vec![];
    for lib in libs {
        let library = match Library::new(&lib) {
            Ok(lib) => lib,
            Err(err) => {
                warn!("Library {} wasn't loaded due error: {}", lib, err);
                continue;
            }
        };
        let plugin_information_callback = match library
            .get::<*mut plugin_interface::PluginInfoCallback>(FN_PLUGIN_INFO)
        {
            Ok(callback) => callback.read(),
            Err(err) => {
                warn!(
                    "Library {} wasn't loaded 
                    because lib doesn't containt valid FN_PLUGIN_INFO function or / and it's signature is incorrect. | {}",
                    lib,
                    err
                );
                continue;
            }
        };

        let boxed_plugin_information = Box::from_raw(plugin_information_callback().cast_mut());

        let str_plugin_name = match CStr::from_ptr(boxed_plugin_information.name).to_str() {
            Ok(res) => res,
            Err(_) => {
                warn!("Plugin not loaded: file '{}' represents a plugin with name, that contains non utf-8 characters.", lib);
                continue;
            }
        };
        let config_ptr = CONFIG
            .plugins
            .config
            .get_key_value(str_plugin_name)
            .map(|(_, v)| extract_config_ptr(v))
            .unwrap_or(ptr::null_mut());

        let state = (boxed_plugin_information.init_callback)(
            config_ptr.cast_const(),
            api_callbacks::get_api(),
        );
        info!("Plugin loaded: {}", str_plugin_name,);

        infos.push(PluginRuntimeInfo {
            _library: library,
            state,
            plugin_information: boxed_plugin_information,
        });

        if !config_ptr.is_null() {
            let _ = CString::from_raw(config_ptr);
        }
    }
    infos
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
