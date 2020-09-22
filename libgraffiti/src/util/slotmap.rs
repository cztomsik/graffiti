use core::marker::PhantomData;
use core::ops::{Index, IndexMut};

pub struct SlotMap<K, V> {
    _phantom: PhantomData<K>,

    // TODO: slot reusing
    slots: Vec<Option<V>>,
}

// could be `From<usize> + Into<usize>` or some `Key` trait but
// we're limited to u32 anyway (because of JS number precision)
// and if it's just 4 bytes, NonZero is probably not worth it either
impl<V> SlotMap<u32, V> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,

            slots: Vec::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (u32, &V)> + '_ {
        self.slots.iter().enumerate().filter_map(|(i, slot)| slot.as_ref().map(|v| (i as u32, v)))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (u32, &mut V)> + '_ {
        self.slots.iter_mut().enumerate().filter_map(|(i, slot)| slot.as_mut().map(|v| (i as u32, v)))
    }

    pub fn slot(&self, key: u32) -> &Option<V> {
        self.slots.get(key as usize).expect("missing slot")
    }

    pub fn slot_mut(&mut self, key: u32) -> &mut Option<V> {
        self.slots.get_mut(key as usize).expect("missing slot")
    }

    pub fn insert(&mut self, value: V) -> u32 {
        self.insert_with_key(|_| value)
    }

    pub fn insert_with_key(&mut self, f: impl FnOnce(u32) -> V) -> u32 {
        let key = std::convert::TryFrom::try_from(self.slots.len()).expect("slotmap full");

        self.slots.push(Some(f(key)));

        key
    }

    pub fn upsert(&mut self, key: u32, value: V) {
        let min_len = key as usize;

        if (min_len > self.slots.len()) {
            self.slots.resize_with(min_len, || None);
        }

        *self.slot_mut(key) = Some(value);
    }

    pub fn remove(&mut self, key: u32) {
        *self.slot_mut(key) = None;
    }
}

impl<V> Index<u32> for SlotMap<u32, V> {
    type Output = V;

    fn index(&self, key: u32) -> &V {
        self.slot(key).as_ref().expect("empty slot")
    }
}

impl<V> IndexMut<u32> for SlotMap<u32, V> {
    fn index_mut(&mut self, key: u32) -> &mut V {
        self.slot_mut(key).as_mut().expect("empty slot")
    }
}
