#![allow(unused_macros)]

use std::num::NonZeroUsize;
use core::fmt::{Debug, Formatter};

// debug logging
// (run cargo with `--features silly,debug`)
macro_rules! debug {
    ($($arg:tt)+) => (
        #[cfg(any(feature = "debug", feature = "silly"))]
        println!($($arg)+);
    )
}

macro_rules! silly {
    ($($arg:tt)+) => (
        #[cfg(feature = "silly")]
        println!($($arg)+);
    )
}

macro_rules! c_str {
    // lit-only, doesn't check for previous \0 but can be used in if/else/match and statics
    ($str:literal) => {
        concat!($str, "\0").as_ptr() as *const c_char
    };

    // BEWARE that if/else/match {} have own scope and so the CString will get emptied!
    ($str:expr) => {
        std::ffi::CString::new($str).expect("invalid CString").as_ptr()
    };
}

// dylib loading

#[macro_use]
pub(crate) mod dylib {
    use std::os::raw::{c_char, c_int, c_void};

    macro_rules! dylib {
        (
            #[$load:tt]
            extern "C" {
                $(fn $fn:ident($($arg:ident: $type:ty),*) $(-> $ret:ty)*;)*
            }
        ) => {
            struct VTable { $( $fn: *mut c_void ),* }
            unsafe impl Sync for VTable {}
            static mut LIB: VTable = VTable { $( $fn: std::ptr::null_mut() ),* };

            unsafe fn $load(file: *const std::os::raw::c_char) {
                let VTable { $($fn),* } = &mut LIB;

                $crate::util::dylib::load_dylib(file, &mut[$((stringify!($fn), $fn)),*]);
            }

            $(
                unsafe fn $fn($($arg: $type),*) $(-> $ret)* {
                    let f: extern "C" fn($($type),*) $(-> $ret)* = std::mem::transmute(LIB.$fn);
                    f($($arg),*)
                }
            )*
        }
    }

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
}

// couldn't use Index<K> because its result is always &V
// so it's not possible to return temp structs
pub trait Lookup<K, V> {
    fn lookup(&self, key: K) -> V;
}

// closures, simple way to get that data from anywhere
impl<K, V, F: Fn(K) -> V> Lookup<K, V> for F {
    #[inline(always)]
    fn lookup(&self, key: K) -> V {
        self(key)
    }
}

// vecs (useful for testing)
impl<V: Clone> Lookup<usize, V> for Vec<V> {
    fn lookup(&self, key: usize) -> V {
        self[key].clone()
    }
}

// generic Id<> so it's a bit harder to mix different indices
pub struct Id<T> {
    id: NonZeroUsize,
    phantom: std::marker::PhantomData<T>
}

impl<T> Id<T> {
    pub(crate) const fn new(id: usize) -> Self {
        Self {
            // new().expect() is not const
            id: unsafe { NonZeroUsize::new_unchecked(id + 1) },
            phantom: std::marker::PhantomData
        }
    }

    #[inline(always)]
    pub(crate) fn index(&self) -> usize {
        self.id.get() - 1
    }
}

// can't derive https://github.com/rust-lang/rust/issues/26925
impl<T> Clone for Id<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            phantom: std::marker::PhantomData
        }
    }
}

// again
impl<T> Copy for Id<T> {}

// and again
impl<T> Debug for Id<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_tuple("").field(&self.id).finish()
    }
}

// and again
impl<T> PartialEq for Id<T> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Id<T> {}

impl<T> std::ops::Index<Id<T>> for Vec<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, id: Id<T>) -> &Self::Output {
        &self[id.index()]
    }
}

impl<T> std::ops::IndexMut<Id<T>> for Vec<T> {
    #[inline(always)]
    fn index_mut(&mut self, id: Id<T>) -> &mut Self::Output {
        &mut self[id.index()]
    }
}
