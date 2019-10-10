#![allow(unused_macros)]

// very basic logging to save some deps
// for now it's enabled/disabled by (un)commenting the line
//
// auto-enabled with --features silly
macro_rules! silly {
    ($($arg:tt)+) => (
        #[cfg(feature = "silly")]
        println!($($arg)+);
    )
}

macro_rules! debug {
    ($($arg:tt)+) => (
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
