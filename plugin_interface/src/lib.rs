use std::{
    ffi::{c_char, c_void},
    ptr,
};

pub type EventCallbalck = unsafe extern "C" fn(*const EventState, ApiCallbacks);
pub type ExecuteCallback = unsafe extern "C" fn(*mut State, ApiCallbacks);
pub type InitCallback = unsafe extern "C" fn(*const c_char, ApiCallbacks) -> *mut State;

pub type PluginInfoCallback = unsafe extern "C" fn() -> *const PluginInformation;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EventState {
    pub state: *const State,
    pub event: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct State {
    pub published_event: *mut c_char,
    pub readable_message: *mut c_char,
    pub human_request: *mut c_char,
    pub data: *const c_void,
}

impl Default for State {
    fn default() -> Self {
        Self {
            published_event: ptr::null_mut(),
            readable_message: ptr::null_mut(),
            human_request: ptr::null_mut(),
            data: ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ApiCallbacks {
    pub send_human_request: unsafe extern "C" fn(*mut c_char),
    pub subscribe_to_events: unsafe extern "C" fn(unsafe extern "C" fn(*const c_char)),
}

#[repr(C)]
#[derive(Debug)]
pub struct PluginInformation {
    pub name: *const c_char,
    pub event_callback: EventCallbalck,
    pub init_callback: InitCallback,
    pub execute_callback: ExecuteCallback,
}
