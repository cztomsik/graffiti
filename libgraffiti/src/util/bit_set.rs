// TODO: I'm not 100% sure if Cell<> is a good idea here
//       because then we might miss some changes called from destructors for example

// like HashSet<u32> but faster
// x grow-only, insert-only, can be cleared
// x safe, backed by bounds-checked Vec<Cell<u32>>

use std::cell::Cell;
use std::num::NonZeroU32;

const BITS: usize = 32;

#[derive(Default)]
pub struct BitSet {
    chunks: Vec<Cell<u32>>,
}

impl BitSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn capacity(&self) -> usize {
        self.chunks.len() * BITS
    }

    pub fn clear(&self) {
        for ch in &self.chunks {
            ch.set(0);
        }
    }

    pub fn add(&self, value: NonZeroU32) {
        let chunk = self.chunk(value);
        chunk.set(chunk.get() | Self::mask(value));
    }

    pub fn remove(&self, value: NonZeroU32) {
        let chunk = self.chunk(value);
        chunk.set(chunk.get() & !Self::mask(value));
    }

    pub fn contains(&self, value: NonZeroU32) -> bool {
        self.chunk(value).get() & Self::mask(value) != 0
    }

    pub fn grow(&mut self, max_value: NonZeroU32) {
        if self.capacity() < max_value.get() as usize {
            let index = max_value.get() as usize / BITS;
            let rem = max_value.get() as usize % BITS;
            self.chunks
                .resize(if rem > 0 { index + 1 } else { index }, Cell::new(0));
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = NonZeroU32> + '_ {
        (1..self.capacity() as u32)
            .into_iter()
            .map(|v| NonZeroU32::new(v).unwrap())
            .filter(move |&v| self.contains(v))
    }

    fn chunk(&self, value: NonZeroU32) -> &Cell<u32> {
        &self.chunks[(value.get() as usize - 1) / BITS]
    }

    fn mask(value: NonZeroU32) -> u32 {
        1 << value.get() % BITS as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn num(n: u32) -> NonZeroU32 {
        NonZeroU32::new(n).unwrap()
    }

    #[test]
    #[should_panic]
    fn grow_needed() {
        let set = BitSet::new();
        set.add(num(1));
    }

    #[test]
    fn grow() {
        let mut set = BitSet::new();

        set.grow(num(32));
        assert_eq!(set.chunks.len(), 1);

        set.grow(num(33));
        assert_eq!(set.chunks.len(), 2);

        set.grow(num(1));
        assert_eq!(set.chunks.len(), 2);
    }

    #[test]
    fn contains() {
        let mut set = BitSet::new();
        set.grow(num(64));

        assert_eq!(set.contains(num(1)), false);

        set.add(num(1));
        assert_eq!(set.contains(num(1)), true);

        set.remove(num(1));
        assert_eq!(set.contains(num(1)), false);
    }

    #[test]
    fn iter() {
        let nums = [3, 5, 8, 13, 21, 34].map(num);

        let mut set = BitSet::new();
        set.grow(num(64));

        for n in nums {
            set.add(n);
        }

        assert_eq!(set.iter().collect::<Vec<_>>(), &nums);
    }

    #[test]
    fn stress() {
        let mut set = BitSet::new();
        set.grow(num(10));

        for n in 1..1000 {
            set.grow(num(n));
            set.add(num(n));
        }

        for n in 1..1000 {
            assert!(set.contains(num(n)));
        }

        for n in [34, 55, 89, 144] {
            set.remove(num(n));
            assert_eq!(set.contains(num(n)), false);
        }
    }
}
