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

macro_rules! offsetof {
    ($ty:ty, $field:ident $(,)?) => ({
        let null: &$ty = core::mem::transmute(ptr::null::<$ty>());
        &null.$field as *const _ as *const std::os::raw::c_void
    });
}

#[macro_use]
mod debug;

#[macro_use]
mod init;

#[macro_use]
mod lazy;

#[macro_use]
pub(crate) mod dylib;

mod atom;

mod id_tree;

mod slotmap;

mod lookup;

mod tree;

pub use tree::Tree;
pub(crate) use tree::TreeAdapter;
pub use dylib::Dylib;
pub use lazy::Lazy;
pub use lookup::Lookup;
pub use slotmap::SlotMap;
pub use id_tree::IdTree;
pub use atom::Atom;
