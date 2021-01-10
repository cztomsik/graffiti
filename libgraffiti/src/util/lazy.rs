use core::cell::UnsafeCell;
use std::sync::Once;

// TODO: dyn? coerce_unsized?
//       https://github.com/rust-lang/rust/issues/27732

pub struct Lazy<T> {
    pub(crate) value: UnsafeCell<Option<T>>,
    pub(crate) once: Once,
    pub(crate) init_fn: fn() -> T,
}

impl<T> Lazy<T> {
    pub fn new(init_fn: fn() -> T) -> Self {
        Self {
            value: UnsafeCell::new(None),
            once: Once::new(),
            init_fn,
        }
    }

    fn inner(&self) -> *mut Option<T> {
        let ptr = self.value.get();

        self.once.call_once(|| unsafe { *ptr = Some((self.init_fn)()) });

        ptr
    }
}

// macro for statics
// (Lazy::new() can't be const because fn pointers are still unstable)
//
// TODO: not sure if it's possible to avoid pub fields (unsafe from_raw_parts() would still need to accept init_fn)
macro_rules! lazy {
    ($init_fn:expr) => {
        Lazy {
            value: core::cell::UnsafeCell::new(None),
            once: std::sync::Once::new(),
            init_fn: $init_fn,
        }
    };
}

// Lazy itself should be safe thanks to Once
unsafe impl<T: Sync> Sync for Lazy<T> {}

impl<T> core::ops::Deref for Lazy<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { (*self.inner()).as_ref().unwrap() }
    }
}
