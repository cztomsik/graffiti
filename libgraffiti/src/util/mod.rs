// TODO: move
macro_rules! c_str {
    // literal-only, doesn't check for previous \0
    ($str:literal) => {
        concat!($str, "\0").as_ptr() as *const std::os::raw::c_char
    };

    // returns Deref<*const c_char> so at least *c_str!("...") works
    ($str:expr) => {
        $crate::util::CStringWrap(
            std::ffi::CString::new($str)
                .expect("invalid CString")
                .into_bytes_with_nul(),
        )
    };
}

// blame rust for this shit
pub struct CStringWrap(pub Vec<u8>);

impl core::ops::Deref for CStringWrap {
    type Target = *const std::os::raw::c_char;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { std::mem::transmute(&self.0) }
    }
}

#[macro_use]
mod init;

#[macro_use]
pub(crate) mod dylib;
pub use dylib::*;

mod atom;
pub use atom::*;

mod slotmap;
pub use slotmap::*;
