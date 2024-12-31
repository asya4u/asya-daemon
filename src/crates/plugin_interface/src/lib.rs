use std::ffi::{c_char, c_void};

pub type EventCallbalck = unsafe extern "C" fn(EventState);
pub type ExecuteCallback = unsafe extern "C" fn(State);
pub type InitCallback = unsafe extern "C" fn(State);

pub type PluginInfo = unsafe extern fn() -> *const PluginInformation;

#[repr(C)]
pub struct EventState {
    pub state: State,
    pub event: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct State {
    pub data: *const c_void
}

#[repr(C)]
#[derive(Debug)]
pub struct PluginInformation {
    pub name: *const c_char,
    pub event_callback: EventCallbalck,
    pub init_callback: InitCallback,
    pub execute_callback: ExecuteCallback,
}
