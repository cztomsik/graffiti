// TODO: check if this actually works

use fnv::FnvHasher;
use std::cell::Cell;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Bloom<T> {
    bits: Cell<u64>,
    marker: PhantomData<T>,
}

impl<T: Hash> Bloom<T> {
    pub fn new() -> Self {
        Self {
            bits: Cell::new(0),
            marker: PhantomData,
        }
    }

    pub fn add(&self, value: &T) {
        let bits = self.bits.get();
        self.bits.set(bits | mask(value));
    }

    #[inline]
    pub fn may_include(&self, value: &T) -> bool {
        let mask = mask(value);
        self.bits.get() & mask == mask
    }

    pub fn clear(&self) {
        self.bits.set(0)
    }
}

#[inline]
fn mask<T: Hash>(v: &T) -> u64 {
    let mut hasher = FnvHasher::with_key(1099511628211);
    v.hash(&mut hasher);
    1 << (hasher.finish() % 64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let bloom = Bloom::new();
        assert!(!bloom.may_include(&12));
        assert!(!bloom.may_include(&34));
        assert!(!bloom.may_include(&56));

        bloom.add(&12);
        bloom.add(&34);

        assert!(bloom.may_include(&12));
        assert!(bloom.may_include(&34));
        assert!(!bloom.may_include(&56));

        bloom.clear();
        assert!(!bloom.may_include(&12));
    }

    #[test]
    fn add_many() {
        let bloom = Bloom::new();

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
