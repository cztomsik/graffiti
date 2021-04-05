// TODO: check how slower RwLock<SlotMap> would be instead of DashMap
// TODO: NonZeroU32 (so more selector parts could be stored inline)

use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::any::{Any, TypeId};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone, Eq)]
pub struct Atom<T: Eq + Hash + 'static>(Arc<T>);

type AtomsOf<T> = DashMap<Arc<T>, ()>;

static ATOMS_OF: Lazy<DashMap<TypeId, Box<dyn Any + Send + Sync>>> = Lazy::new(|| DashMap::new());

impl<T: 'static + Eq + Hash + Send + Sync> From<T> for Atom<T> {
    fn from(v: T) -> Self {
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

// conv helper
// TODO: ToOwned vs. conflicts?
impl From<&str> for Atom<String> {
    fn from(v: &str) -> Self {
        Self::from(v.to_owned())
    }
}

impl<T: Eq + Hash> Deref for Atom<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.0.as_ref()
    }
}

impl<T: Eq + Hash + 'static> Drop for Atom<T> {
    fn drop(&mut self) {
        let atoms = ATOMS_OF.get(&TypeId::of::<T>()).unwrap();
        let atoms = atoms.downcast_ref::<AtomsOf<T>>().unwrap();

        // the one which is dropped + shared dashmap
        atoms.remove_if(&self.0, |k, _| Arc::strong_count(k) == 2);
    }
}

impl<T: Eq + Hash> Hash for Atom<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        Arc::as_ptr(&self.0).hash(hasher)
    }
}

impl<T: Eq + Hash> PartialEq for Atom<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::as_ptr(&self.0) == Arc::as_ptr(&other.0)
    }
}
