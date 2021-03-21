#![allow(unused_macros)]

// like Index<K> but it's possible to return temp structs
pub trait Lookup<K, V> {
    fn lookup(&self, key: K) -> V;
}

// closures, simple way to get that data from anywhere
impl<K, V, F: Fn(K) -> V> Lookup<K, V> for F {
    #[inline(always)]
    fn lookup(&self, key: K) -> V {
        self(key)
    }
}

// vecs (useful for testing)
impl<V: Clone> Lookup<usize, V> for Vec<V> {
    fn lookup(&self, key: usize) -> V {
        self[key].clone()
    }
}
