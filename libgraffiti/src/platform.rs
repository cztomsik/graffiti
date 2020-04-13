// TODO: split to separate things, keeping everything in platform/* was bad idea

// (not just) platform-dependent stuff:
// - dylib loading
// - windowing
// - image loading using available system-wide libraries (TODO)
// - font loading ... (TODO)

use crate::app::WindowEvent;
use crate::viewport::GlViewport;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

pub unsafe fn load_dylib(file: *const c_char, symbols: &mut [(&str, &mut *mut c_void)]) {
    #[cfg(target_family = "unix")]
    let handle = dlopen(file, RTLD_NOW);

    #[cfg(target_family = "windows")]
    let handle = LoadLibraryA(file);

    if handle == std::ptr::null_mut() {
        panic!("load lib {:?}", std::ffi::CStr::from_ptr(file));
    }

    for (name, ptr) in symbols {
        #[cfg(target_family = "unix")]
        let addr = dlsym(handle, c_str!(*name));

        #[cfg(target_os = "windows")]
        let addr = GetProcAddress(handle, c_str!(*name));

        if addr == std::ptr::null_mut() {
            panic!("load fn {} in lib {:?}", name, std::ffi::CStr::from_ptr(file));
        }

        **ptr = addr;
    }
}

pub fn dylib_file(name: &str, ver: &str) -> String {
    if cfg!(target_os = "windows") {
        format!("{}{}.dll", name, ver)
    } else if cfg!(target_os = "macos") {
        format!("lib{}.{}.dylib", name, ver)
    } else {
        format!("lib{}.so.{}", name, ver)
    }
}

// TODO RTLD_NOW is 0 on android
#[cfg(target_family = "unix")]
const RTLD_NOW: c_int = 2;

#[cfg(target_family = "unix")]
extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

#[cfg(target_os = "windows")]
extern "C" {
    fn LoadLibraryA(filename: *const c_char) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const c_char) -> *mut c_void;
}

// pointer to the opaque, platform-specific data type
pub type NativeWindow = *mut std::os::raw::c_void;

// plumbing needed for event handling
// set by `App.get_events()`
pub static mut WINDOWS_PTR: *mut Vec<NativeWindow> = ptr::null_mut();
pub static mut VIEWPORTS_PTR: *mut Vec<GlViewport> = ptr::null_mut();
pub static mut PENDING_EVENTS_PTR: *mut Vec<WindowEvent> = ptr::null_mut();

// we provide this to respective implementations so that they don't need to
// mess with own event abstractions (they'll just call respective `GlViewport` method directly)
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
    }};
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
