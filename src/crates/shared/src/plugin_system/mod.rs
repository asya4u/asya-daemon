use libloading::Library;
use log::debug;
use plugin_interface::{EventState, PluginInformation, State};
use serde::Serialize;
use std::{
    ffi::{CStr, CString},
    fs, io,
    path::Path,
    ptr::{self},
    thread,
    time::Duration,
};
use tokio::sync::{mpsc::Receiver, Mutex};

use crate::{configuration::CONFIG, event_system};

// todo: редизайн типов чтобы такой хуеты как с Library не было
// !! порядок полей менять НЕЛЬЗЯ тоже может быть сегфолт
struct PluginRuntimeInfo {
    plugin_information: Box<PluginInformation>,
    _library: Library, // это поле вообще никгде не юзается, но без него сегфолт.
    state: *mut State,
}

pub fn load_plugins(receiver: Mutex<Receiver<String>>) {
    unsafe {
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let libs = find_plugins();
                dbg!(&libs);

                let mut plugins_data = load_plugin_data(libs);

                run_inits(&mut plugins_data);
                poll(&mut plugins_data, receiver).await
            })
        })
    };
}

fn find_plugins() -> Vec<String> {
    let plugins_folder = &CONFIG.plugins.plugins_folder;
    let extension = if cfg!(target_family = "unix") {
        "so" // sal?
    } else {
        "dll"
    };

    find_files_with_extension(Path::new(plugins_folder), extension).unwrap() //todo
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

#[derive(Debug, Serialize)]
pub struct PluginEvent {
    sender: String,
    data: String,
}

async unsafe fn poll(plugins_data: &mut [PluginRuntimeInfo], receiver: Mutex<Receiver<String>>) {
    let mut recv = receiver.lock().await;
    loop {
        for info in &mut *plugins_data {
            if !recv.is_empty() {
                let event_callback = info.plugin_information.event_callback;
                let res = recv.recv().await;
                let ptr = CString::new(res.unwrap()).unwrap();
                let event_state = Box::into_raw(Box::new(EventState {
                    state: info.state,
                    event: ptr.as_ptr(),
                }));
                (event_callback)(event_state);
            } else {
                let execute_callback = info.plugin_information.execute_callback;
                (execute_callback)(info.state);
            }
            if !info.state.is_null() {
                let state = *info.state;
                if !state.published_event.is_null() {
                    let event = state.published_event;
                    let data = CStr::from_ptr(event)
                        .to_str()
                        .unwrap_or_default()
                        .to_string();
                    let event = PluginEvent {
                        sender: CStr::from_ptr(info.plugin_information.name)
                            .to_str()
                            .unwrap()
                            .to_string(),
                        data,
                    };
                    event_system::publish(event).await;
                }
                drop(Box::from_raw((*info.state).published_event));
                (*info.state).published_event = ptr::null_mut()
            }
        }
        tokio::time::sleep(Duration::from_micros(1_000)).await;
    }
}

unsafe fn load_plugin_data(libs: Vec<String>) -> Vec<PluginRuntimeInfo> {
    const FN_PLUGIN_INFO: &[u8; 11] = b"plugin_info";
    let mut infos = vec![];
    for lib in libs {
        let library = Library::new(lib).unwrap();
        let plugin_information = library
            .get::<*mut plugin_interface::PluginInfoCallback>(FN_PLUGIN_INFO)
            .expect("lib is not loaded")
            .read();

        let plugin_information = Box::from_raw(plugin_information().cast_mut());

        infos.push(PluginRuntimeInfo {
            _library: library,
            state: ptr::null_mut(),
            plugin_information,
        });
    }
    infos
}

unsafe fn run_inits(infos: &mut Vec<PluginRuntimeInfo>) {
    for info in infos {
        info.state = (info.plugin_information.init_callback)();
    }
}
