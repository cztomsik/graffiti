#![allow(unused_macros)]

// very basic logging to save some deps
//
// enable with --features silly,debug
macro_rules! silly {
    ($($arg:tt)+) => (
        #[cfg(feature = "silly")]
        println!($($arg)+);
    )
}

macro_rules! debug {
    ($($arg:tt)+) => (
        #[cfg(any(feature = "debug", feature = "silly"))]
        println!($($arg)+);
    )
}

macro_rules! error {
    ($($arg:tt)+) => (
        eprintln!($($arg)+);
    )
}

// function is not enough because the string is freed before the pointer could be used
// BEWARE that if/else/match {} have own scope and so the CString will get emptied!
macro_rules! c_str {
    ($str:expr) => {
        std::ffi::CString::new($str).expect("invalid CString").as_ptr()
    };
}

// dylib loading
macro_rules! dylib {
    (
        #[$load:tt]
        extern "C" {
            $(
                fn $fn:ident($($arg:ident: $type:ty),*) $(-> $ret:ty)*;
            )*
        }
    ) => {
        struct VTable {
            $( $fn: *mut c_void ),*
        }

        unsafe impl Sync for VTable {}

        static mut LIB: VTable = VTable {
            $( $fn: std::ptr::null_mut() ),*
        };

        unsafe fn $load(file: *const std::os::raw::c_char) {
            let VTable { $($fn),* } = &mut LIB;

            crate::platform::load_dylib(file, &mut[
                $( (stringify!($fn), $fn) ),*
            ]);
        }

        $(
            unsafe fn $fn($($arg: $type),*) $(-> $ret)* {
                let f: extern "C" fn($($type),*) $(-> $ret)* = std::mem::transmute(LIB.$fn);
                f($($arg),*)
            }
        )*
    }
}
