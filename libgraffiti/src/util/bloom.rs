// so the idea was to store bloom of all ancestors in each node
// so we could easily bit-check if any of the dirty nodes was
// an ancestor of some other node and this way sort-of quickly figure
// out what needs to be re-evaluated (because of inherited props)
//
// I thought it would be a good trick because bloom is Copy and
// we only need to set it after insert and we only visit the tree
// downwards so the updates should be quick
//
// but now when I think about all of this the problem is that
// bloom gets worse with number of ancestors and at around
// 14 we would get 1-in-8 false positives, which is still good but
// maybe bloom is not a good fit because the more deeper the node is,
// the worse false-rate it gets (and gets re-computed for every change)
//
// whereas if we had just "dumb" dirty flags we would need to propagate
// down for every change but we can also short-circuit if the subtree is
// already dirty so maybe it's not such a big deal. and we could fast-reject
// EVERYTHING
//
// another use-case might be "bitmasking" selector tails to
// quick-reject rules during style matching but I'm not sure if
// it's not better to use special-purpose bitmask for that rather than
// trying to fit generic bloom for that
//
// also, it's a question if we want to have multiple masks or just
// do sort of "union check" rule & selectors != 0
//
// the whole idea is that filtering masks sequentially is very quick
// and if it can shave some work, then it's definitely worth
// but of course it's better to not having to re-compute styles at all
// and that sort of goes back to the original bloom vs. dirty flags thing.
//
// note: servo & webkit are also using bloom filter for selector matching
// but they are using counting bloom filter which can also "remove" items,
// and their bloom is huge (2KB) so it actually works with very very small
// false-positive rate and it sort of replaces whole ancestor matching
// which we might consider doing as well eventually but maybe we'll be fine
// with something simpler

use fnv::FnvHasher;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

const BITS: u64 = 64;

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

    pub fn with(self, value: &T) -> Self {
        Self {
            bits: self.bits | mask(value),
            marker: PhantomData,
        }
    }

    pub fn may_include(self, value: &T) -> bool {
        let mask = mask(value);
        self.bits & mask == mask
    }
}

impl<T> fmt::Debug for Bloom<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Bloom").finish()
    }
}

impl<T: Hash> Default for Bloom<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for Bloom<T> {
    fn clone(&self) -> Self {
        Self {
            bits: self.bits,
            marker: PhantomData,
        }
    }
}

impl<T> Copy for Bloom<T> {}

impl<T> PartialEq for Bloom<T> {
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits
    }
}

impl<T> Eq for Bloom<T> {}

impl<T> Hash for Bloom<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.bits.hash(hasher);
    }
}

// TODO: use two hashes (or better hash fn)
fn mask<T: Hash>(v: &T) -> u64 {
    let mut hasher = FnvHasher::default();
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
    fn false_positives() {
        let mut hits = 0;
        let mut bloom = Bloom::new();
        bloom.add(&1);
        bloom.add(&9);
        bloom.add(&17);
        bloom.add(&43);

        for n in 0..100 {
            if bloom.may_include(&n) {
                hits += 1;
            }
        }

        // 7 hits, 4 are there
        // we have rejected 93 but there were 3 misses (extra work)
        assert_eq!(hits, 7);

        let mut hits = 0;
        let mut bloom = Bloom::new();

        // insert 1/3 of 100 ids
        for n in 0..100 {
            if n % 3 == 0 {
                bloom.add(&n);
            }
        }

        for n in 0..100 {
            if bloom.may_include(&n) {
                hits += 1;
            }
        }

        // insert 33, should reject 67 but it only
        // rejected 42, which means we will do extra
        // work for 25 items
        //
        // TODO: we should be able to get closer to this
        // https://hur.st/bloomfilter/?n=14&p=&m=64&k=1
        assert_eq!(hits, 0);
    }

    #[test]
    fn mask_truthy() {
        for n in 0..1_000_000 {
            assert!(mask(&n) != 0);
        }
    }
}
