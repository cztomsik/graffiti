use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::slice::Iter;
use std::iter::FromIterator;

pub struct DenseStorage<K, V>(Vec<V>, PhantomData<K>);

impl <K, V> DenseStorage<K, V> where K: DenseKey {
    pub fn new() -> Self {
        DenseStorage(vec![], PhantomData)
    }

    pub fn push(&mut self, value: V) {
        self.insert(value);
    }

    // TODO: return K: DenseKey
    pub fn insert(&mut self, value: V) -> usize {
        let index = self.0.len();
        self.0.push(value);

        index
    }

    pub fn get(&self, key: K) -> &V {
        &self.0[key.to_index()]
    }

    pub fn get_mut(&mut self, key: K) -> &mut V {
        &mut self.0[key.to_index()]
    }

    // mutably borrow two items at once
    pub fn get_two_muts<'a>(&mut self, first: K, second: K) -> (&'a mut V, &'a mut V) {
        let len = self.0.len();
        let first= first.to_index();
        let second= second.to_index();

        assert!(first < len);
        assert!(second < len);
        assert_ne!(first, second);

        let ptr = self.0.as_mut_ptr();

        unsafe {
            (&mut *ptr.add(first), &mut *ptr.add(second))
        }
    }

    // unused
    //pub fn set(&mut self, key: K, value: V) {
    //    self.0[key.to_index()] = value;
    //}

    pub fn iter(&self) -> Iter<V> {
        self.0.iter()
    }
}

impl <K, V> FromIterator<V> for DenseStorage<K, V> where K: DenseKey {
    fn from_iter<T: IntoIterator<Item=V>>(iter: T) -> Self {
        let mut storage = DenseStorage::new();

        for it in iter {
            storage.push(it);
        }

        storage
    }
}

pub trait DenseKey {
    fn to_index(&self) -> usize;
}

impl DenseKey for usize {
    fn to_index(&self) -> usize {
        *self
    }
}

impl DenseKey for u32 {
    fn to_index(&self) -> usize {
        *self as usize
    }
}

impl DenseKey for u16 {
    fn to_index(&self) -> usize {
        *self as usize
    }
}

pub struct SparseStorage<K, V>(BTreeMap<K, V>);

impl <K, V> SparseStorage<K, V> where K: Ord {
    pub fn new() -> Self {
        SparseStorage(BTreeMap::new())
    }

    pub fn get(&self, key: K) -> Option<&V> {
        self.0.get(&key)
    }

    pub fn set(&mut self, key: K, value: Option<V>) {
        if let Some(value) = value {
            self.0.insert(key, value);
        } else {
            self.0.remove(&key);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dense() {
        let key: usize = 0;
        let mut storage = DenseStorage::new();
        storage.push(false);

        assert_eq!(storage.get(key), &false);

        //storage.set(key, true);
        //assert_eq!(storage.get(key), &true);
    }

    #[test]
    fn test_sparse() {
        let key: usize = 0;
        let mut storage = SparseStorage::new();

        assert_eq!(storage.get(key), None);

        storage.set(key, Some(true));
        assert_eq!(storage.get(key), Some(&true));
    }
}
