mod atom;
mod bloom;
mod slotmap;

use std::ops::Index;

// construct Index<> impl from &T and closure which takes
// the source and key and returns the value
// TODO: rename? different api? trait?
//       right now it's sometimes necessary to typehint the closure key
//       maybe with different api this wouldn't be a problem?
pub fn index_with<'a, T, K: 'static, V: 'static + ?Sized>(
    source: &'a T,
    fun: impl Fn(&'a T, K) -> &'a V + 'a,
) -> impl Index<K, Output = V> + 'a {
    IndexWrap(source, fun)
}

pub struct IndexWrap<'a, T, F>(&'a T, F);

impl<'a, T, K: 'static, V: 'static + ?Sized, F: Fn(&'a T, K) -> &'a V> Index<K> for IndexWrap<'a, T, F> {
    type Output = V;

    fn index(&self, index: K) -> &V {
        self.1(self.0, index)
    }
}

pub(crate) use self::{
    atom::Atom,
    bloom::Bloom,
    slotmap::{Key, SlotMap},
};
