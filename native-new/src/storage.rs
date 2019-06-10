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
