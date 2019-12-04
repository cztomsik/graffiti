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
        #[cfg(feature = "debug")]
        println!($($arg)+);
    )
}

macro_rules! error {
    ($($arg:tt)+) => (
        eprintln!($($arg)+);
    )
}

// function is not enough because the string is freed before the pointer could be used
macro_rules! c_str {
    ($str:expr) => {
        std::ffi::CString::new($str).expect("invalid CString").as_ptr()
    }
}
