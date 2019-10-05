#![allow(unused_macros)]

// very basic logging to save some deps
// for now it's enabled/disabled by (un)commenting the line
//
// TODO: auto-enable based on some env var
macro_rules! silly {
    ($($arg:tt)+) => (
        // println!($($arg)+);
    )
}

macro_rules! debug {
    ($($arg:tt)+) => (
        println!($($arg)+);
    )
}

macro_rules! error {
    ($($arg:tt)+) => (
        println!($($arg)+);
    )
}

// function is not enough because the string is freed before the pointer could be used
macro_rules! c_str {
    ($str:expr) => {
        std::ffi::CString::new($str).expect("invalid CString").as_ptr()
    }
}
