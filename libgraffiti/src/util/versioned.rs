// immutable value with (unique enough) version
// so it's cheap to compare

use core::sync::atomic::AtomicU32;

pub struct Versioned<T> {
    // TODO: nonzero?
    version: u32,
    value: T,
}

impl<T> Versioned<T> {
    pub fn new(value: T) -> Self {
        static NEXT_VERSION: AtomicU32 = AtomicU32::new(1);

        Self {
            version: NEXT_VERSION.fetch_add(1, core::sync::atomic::Ordering::SeqCst),
            value,
        }
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn map<B, F>(&self, f: F) -> Versioned<B> where F: FnOnce(&Self) -> B {
        Versioned {
            version: self.version,
            value: f(self)
        }
    }
}

impl<T: Clone> Versioned<T> {
    pub fn with(&self, mut f: impl FnMut(&mut T)) -> Self {
        let mut value = self.value.clone();

        f(&mut value);

        Self::new(value)
    }
}

impl<T> core::ops::Deref for Versioned<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}
