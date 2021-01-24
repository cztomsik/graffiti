// TODO: check how slower RwLock<SlotMap> would be instead of DashMap
// TODO: NonZeroU32 (so more selector parts could be stored inline)

use crate::util::Lazy;
use core::any::{Any, TypeId};
use core::hash::Hash;
use core::ops::Deref;
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Atom<T: Eq + Hash + Send + Sync + 'static>(Arc<T>);

type AtomsOf<T> = DashMap<Arc<T>, ()>;

static ATOMS_OF: Lazy<DashMap<TypeId, Box<dyn Any + Send + Sync>>> = lazy!(|| DashMap::new());

impl<T: 'static + Eq + Hash + Send + Sync> Atom<T> {
    pub fn new(v: T) -> Self {
        // TODO: static var per generic type wouldn't work, rust
        //       accepts the syntax but it's shared for all types (WTF)
        //       https://github.com/rust-lang/rust/issues/22991

        let type_id = TypeId::of::<T>();
        let atoms = match ATOMS_OF.get(&type_id) {
            Some(set) => set,
            None => ATOMS_OF
                .entry(type_id)
                .or_insert_with(|| Box::new(AtomsOf::<T>::new()))
                .downgrade(),
        };

        let atoms: &AtomsOf<T> = atoms.value().downcast_ref::<AtomsOf<T>>().unwrap();
        let entry = atoms.entry(Arc::new(v)).or_insert(());
        return Self(entry.key().clone());
    }
}

impl<T: Eq + Hash + Send + Sync> Deref for Atom<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.0.as_ref()
    }
}

impl<T: 'static + Eq + Hash + Send + Sync> Drop for Atom<T> {
    fn drop(&mut self) {
        let atoms = ATOMS_OF.get(&TypeId::of::<T>()).unwrap();
        let atoms = atoms.downcast_ref::<AtomsOf<T>>().unwrap();

        // the one which is dropped + shared dashmap
        atoms.remove_if(&self.0, |k, _| Arc::strong_count(k) == 2);
    }
}
