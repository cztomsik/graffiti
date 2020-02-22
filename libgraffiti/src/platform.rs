// (not just) platform-dependent stuff:
// - windowing
// - image loading using available system-wide libraries (TODO)
// - font loading ... (TODO)

use crate::app::{WindowEvent};
use crate::viewport::{Viewport};
use std::ptr;

// pointer to the opaque, platform-specific data type
pub type NativeWindow = *mut std::os::raw::c_void;

// plumbing needed for event handling
// set by `App.get_events()`
pub static mut WINDOWS_PTR: *mut Vec<NativeWindow> = ptr::null_mut();
pub static mut VIEWPORTS_PTR: *mut Vec<Viewport> = ptr::null_mut();
pub static mut PENDING_EVENTS_PTR: *mut Vec<WindowEvent> = ptr::null_mut();

// we provide this to respective implementations so that they don't need to
// mess with own event abstractions (they'll just call respective `Viewport` method directly)
// 
// function is not enough because the closure captures the args
macro_rules! window_event {
    ($w:ident, $body:expr) => {{
        for (id, wnd) in (*crate::platform::WINDOWS_PTR).iter_mut().enumerate() {
            if wnd == &($w as *mut c_void) {
                let $w = &mut (*crate::platform::VIEWPORTS_PTR)[id];

                let event = $body;
                (*crate::platform::PENDING_EVENTS_PTR).push(WindowEvent { window: id, event });
            }
        }
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

// windows
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;
