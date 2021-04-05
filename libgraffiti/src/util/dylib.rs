// dylib utils

use std::os::raw::{c_char, c_int, c_void};

// wrapper around dlopen/...
pub struct Dylib {
    handle: *mut c_void,
}

unsafe impl Sync for Dylib {}

impl Dylib {
    pub fn load(filename: *const c_char) -> Self {
        unsafe {
            #[cfg(target_family = "unix")]
            let handle = dlopen(filename, RTLD_NOW);

            #[cfg(target_family = "windows")]
            let handle = LoadLibraryA(filename);

            // TODO: Result
            if handle == std::ptr::null_mut() {
                panic!("load lib {:?}", std::ffi::CStr::from_ptr(filename));
            }

            Self { handle }
        }
    }

    pub fn symbol(&self, name: *const c_char) -> *mut c_void {
        unsafe {
            #[cfg(target_family = "unix")]
            return dlsym(self.handle, name);

            #[cfg(target_family = "windows")]
            return GetProcAddress(self.handle, name);
        }
    }
}

impl Drop for Dylib {
    fn drop(&mut self) {
        unsafe {
            #[cfg(target_family = "unix")]
            dlclose(self.handle);

            #[cfg(target_family = "windows")]
            FreeLibrary(self.handle);
        }
    }
}

// TODO: RTLD_NOW is 0 on x32 android
#[cfg(target_family = "unix")]
const RTLD_NOW: c_int = 2;

#[cfg(target_family = "unix")]
extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
}

#[cfg(target_os = "windows")]
extern "C" {
    fn LoadLibraryA(filename: *const c_char) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const c_char) -> *mut c_void;
    fn FreeLibrary(handle: *mut c_void) -> c_int;
}

// generate vtable + (unsafe) fns
// - once per module
// - fns are unsafe because we don't know if they are mem-safe, thread-safe or even loaded at all
// - load_with() is unsafe because somebody could be reading already (but each set should be atomic)
//   - we need it because sometimes
macro_rules! dylib {
    (
        extern $("C")? {
            $($pub:vis fn $fn:ident($($arg:ident: $type:ty),*) $(-> $ret:ty)*;)*
        }
    ) => {
        struct __LibFns { $( $fn: *mut c_void ),* }
        unsafe impl Sync for __LibFns {}
        static mut __LIB_FNS: __LibFns = __LibFns { $( $fn: crate::util::dylib::__not_loaded as _ ),* };

        pub(crate) unsafe fn load_with(mut load_symbol: impl FnMut(&str) -> *mut c_void) {
            $(__LIB_FNS.$fn = load_symbol(stringify!($fn));)*
        }

        $(
            $pub unsafe fn $fn($($arg: $type),*) $(-> $ret)* {
                let f: extern "C" fn($($type),*) $(-> $ret)* = std::mem::transmute(__LIB_FNS.$fn);
                f($($arg),*)
            }
        )*
    }
}

#[inline(never)]
pub(crate) extern "C" fn __not_loaded() {
    panic!("not loaded")
}
