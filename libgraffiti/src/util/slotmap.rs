// slotmap reimplementation with u32 and without versioning
// the reason is that V8 doesn't like numbers above 2^30

use std::num::NonZeroU32;
use std::ops::{Index, IndexMut};

pub struct SlotMap<K, V> {
    slots: Vec<Option<V>>,
    free_keys: Vec<K>,
}

// could be `From<usize> + Into<usize>` or some `Key` trait but
// we're limited to u32 anyway (because of JS number precision and SMIs)
impl<V> SlotMap<NonZeroU32, V> {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            free_keys: Vec::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (NonZeroU32, &V)> + '_ {
        self.slots
            .iter()
            .enumerate()
            .filter_map(|(i, slot)| slot.as_ref().map(|v| (NonZeroU32::new(i as u32 + 1).unwrap(), v)))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (NonZeroU32, &mut V)> + '_ {
        self.slots
            .iter_mut()
            .enumerate()
            .filter_map(|(i, slot)| slot.as_mut().map(|v| (NonZeroU32::new(i as u32 + 1).unwrap(), v)))
    }

    pub fn slot(&self, key: NonZeroU32) -> &Option<V> {
        self.slots.get(key.get() as usize - 1).expect("missing slot")
    }

    pub fn slot_mut(&mut self, key: NonZeroU32) -> &mut Option<V> {
        self.slots.get_mut(key.get() as usize - 1).expect("missing slot")
    }

    pub fn insert(&mut self, value: V) -> NonZeroU32 {
        if let Some(key) = self.free_keys.pop() {
            self.slots[key.get() as usize - 1] = Some(value);
            key
        } else {
            self.slots.push(Some(value));
            NonZeroU32::new(self.slots.len() as _).unwrap()
        }
    }

    pub fn put(&mut self, key: NonZeroU32, value: V) {
        // TODO: full
        let min_len = key.get() as usize - 1 + 1;
        if min_len > self.slots.len() {
            self.slots.resize_with(min_len, || None);
        }

        *self.slot_mut(key) = Some(value);
    }

    pub fn remove(&mut self, key: NonZeroU32) -> Option<V> {
        self.free_keys.push(key);
        self.slot_mut(key).take()
    }
}

impl<V> Default for SlotMap<NonZeroU32, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V> Index<NonZeroU32> for SlotMap<NonZeroU32, V> {
    type Output = V;

    fn index(&self, key: NonZeroU32) -> &V {
        self.slot(key).as_ref().expect("empty slot")
    }
}

impl<V> IndexMut<NonZeroU32> for SlotMap<NonZeroU32, V> {
    fn index_mut(&mut self, key: NonZeroU32) -> &mut V {
        self.slot_mut(key).as_mut().expect("empty slot")
    }
}
