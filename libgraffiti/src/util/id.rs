use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::ops::{Deref, DerefMut};

#[repr(transparent)]
pub struct Id<T>(NonZeroU32, PhantomData<T>);

impl<T> Id<T> {
    pub(crate) fn new(num: NonZeroU32) -> Self {
        Self(num, PhantomData)
    }
}

impl<T> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Id").field(&self.0).finish()
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Id(self.0, PhantomData)
    }
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> Deref for Id<T> {
    type Target = NonZeroU32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Id<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
