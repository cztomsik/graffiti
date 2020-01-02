// (not just) platform-dependent stuff:
// - windowing
// - image loading using available system-wide libraries

use std::collections::BTreeMap;
use crate::viewport::{Viewport, Event};
use std::ptr;

// pointer to the opaque, platform-specific data type
pub type NativeWindow = *mut std::os::raw::c_void;

// plumbing needed for event handling
// set by `App.get_events()`
pub static mut WINDOWS_PTR: *mut BTreeMap<NativeWindow, Viewport> = ptr::null_mut();
pub static mut PENDING_EVENTS_PTR: *mut Vec<Event> = ptr::null_mut();

// we provide this to respective implementations so that they don't need to
// mess with own event abstractions (they'll just call respective `Viewport` method directly)
// 
// function is not enough because the closure captures the args
macro_rules! window_event {
    ($w:ident, $body:expr) => {{
        // TODO: multi-window
        let $w = (*WINDOWS_PTR).get_mut(&($w as *mut c_void)).expect("missing window");
        let event = $body;

        (*PENDING_EVENTS_PTR).push(event);
    }}
}

// shared utils
mod glfw;

// supported platforms
// unfortnutaly it's not possible to put both statements in one conditional
// it  might be possible to do with `mod internal { mod x; pub use x::*; }`
// but then we'd need another directory level

// macos
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

// linux
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;
