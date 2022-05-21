// slotmap without versioning
// (V8 doesn't like numbers above 2^30)

use std::num::NonZeroU32;
use std::ops::{Index, IndexMut};

pub struct SlotMap<K, V> {
    slots: Vec<Option<V>>,
    free_keys: Vec<K>,
}

impl<K: Key, V> SlotMap<K, V> {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            free_keys: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.slots.clear();
        self.free_keys.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = (K, &V)> + '_ {
        self.slots
            .iter()
            .enumerate()
            .filter_map(|(i, slot)| slot.as_ref().map(|v| (K::from_index(i).unwrap(), v)))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (K, &mut V)> + '_ {
        self.slots
            .iter_mut()
            .enumerate()
            .filter_map(|(i, slot)| slot.as_mut().map(|v| (K::from_index(i).unwrap(), v)))
    }

    pub fn slot(&self, key: K) -> &Option<V> {
        self.slots.get(key.index()).expect("missing slot")
    }

    pub fn slot_mut(&mut self, key: K) -> &mut Option<V> {
        self.slots.get_mut(key.index()).expect("missing slot")
    }

    pub fn insert(&mut self, value: V) -> K {
        if let Some(key) = self.free_keys.pop() {
            self.slots[key.index()] = Some(value);
            key
        } else {
            let key = K::from_index(self.slots.len()).expect("slotmap full");
            self.slots.push(Some(value));

            key
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        let min_len = key.index() + 1;
        if min_len > self.slots.len() {
            self.slots.resize_with(min_len, || None);
        }

        *self.slot_mut(key) = Some(value);
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        self.free_keys.push(key);
        self.slot_mut(key).take()
    }
}

impl<K: Key, V> Default for SlotMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Key, V> Index<K> for SlotMap<K, V> {
    type Output = V;

    fn index(&self, key: K) -> &V {
        self.slot(key).as_ref().expect("empty slot")
    }
}

impl<K: Key, V> IndexMut<K> for SlotMap<K, V> {
    fn index_mut(&mut self, key: K) -> &mut V {
        self.slot_mut(key).as_mut().expect("empty slot")
    }
}

pub trait Key: Copy {
    fn from_index(index: usize) -> Option<Self>;
    fn index(self) -> usize;
}

impl Key for NonZeroU32 {
    fn from_index(index: usize) -> Option<Self> {
        Some(Self::new(index as u32 + 1)?)
    }

    fn index(self) -> usize {
        self.get() as usize - 1
    }
}
