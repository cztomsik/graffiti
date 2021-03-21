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

macro_rules! offsetof {
    ($ty:ty, $field:ident $(,)?) => {{
        let null: &$ty = core::mem::transmute(ptr::null::<$ty>());
        &null.$field as *const _ as *const std::os::raw::c_void
    }};
}

#[macro_use]
mod init;

#[macro_use]
pub(crate) mod dylib;
pub use dylib::*;

mod atom;
pub use atom::*;

mod id_tree;
pub use id_tree::*;

mod lookup;
pub use lookup::*;

mod slotmap;
pub use slotmap::*;

mod tree;
pub use tree::*;
