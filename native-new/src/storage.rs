use std::collections::BTreeMap;

pub trait Storage<K, V> {
    fn set(&mut self, key: K, value: V);
}

impl<K, V> Storage<K, Option<V>> for BTreeMap<K, V> where K: Ord {
    fn set(&mut self, key: K, value: Option<V>) {
        if let Some(value) = value {
            self.insert(key, value);
        } else {
            self.remove(&key);
        }
    }
}

/*
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
*/
