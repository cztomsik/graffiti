// TODO: move to c_str/ffi?
macro_rules! c_str {
    // lit-only, doesn't check for previous \0 but can be used in if/else/match and statics
    ($str:literal) => {
        concat!($str, "\0").as_ptr() as *const std::os::raw::c_char
    };

    // BEWARE that if/else/match {} have own scope and so the CString will get emptied!
    ($str:expr) => {
        std::ffi::CString::new($str).expect("invalid CString").as_ptr()
    };
}

/*
macro_rules! c_string {
    ($str:expr) => {
        std::ffi::CString::new($str).expect("invalid CString")
    };
}
*/

#[macro_use]
mod debug;

#[macro_use]
mod init;

#[macro_use]
mod lazy;

#[macro_use]
pub mod dylib;

mod slotmap;

pub use dylib::Dylib;
pub use lazy::Lazy;
pub use slotmap::SlotMap;
