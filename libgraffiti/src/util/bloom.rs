// TODO: check if this actually works

use fnv::FnvHasher;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

const BITS: u64 = 64;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bloom<T> {
    bits: u64,
    marker: PhantomData<T>,
}

impl<T: Hash> Bloom<T> {
    pub const EMPTY: Self = Self {
        bits: 0,
        marker: PhantomData,
    };
    pub const MAX: Self = Self {
        bits: u64::MAX,
        marker: PhantomData,
    };

    pub fn new() -> Self {
        Self {
            bits: 0,
            marker: PhantomData,
        }
    }

    pub fn add(&mut self, value: &T) {
        *self = self.with(value);
    }

    pub fn with(&self, value: &T) -> Self {
        Self {
            bits: self.bits | mask(value),
            marker: PhantomData,
        }
    }

    pub fn may_include(&self, value: &T) -> bool {
        let mask = mask(value);
        self.bits & mask == mask
    }
}

impl<T: Hash> Default for Bloom<T> {
    fn default() -> Self {
        Self::new()
    }
}

fn mask<T: Hash>(v: &T) -> u64 {
    let mut hasher = FnvHasher::with_key(1099511628211);
    v.hash(&mut hasher);
    1 << (hasher.finish() % BITS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut bloom = Bloom::new();
        assert!(!bloom.may_include(&12));
        assert!(!bloom.may_include(&34));
        assert!(!bloom.may_include(&56));

        bloom.add(&12);
        bloom.add(&34);

        assert!(bloom.may_include(&12));
        assert!(bloom.may_include(&34));
        assert!(!bloom.may_include(&56));
    }

    #[test]
    fn add_many() {
        let mut bloom = Bloom::new();

        for n in 0..1_000_000 {
            bloom.add(&n);
            assert!(bloom.may_include(&n));
        }
    }

    #[test]
    fn mask_truthy() {
        for n in 0..1_000_000 {
            assert!(mask(&n) != 0);
        }
    }
}
