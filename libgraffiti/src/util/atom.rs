// TODO: NonZeroU32 (so more selector parts could be stored inline)

use once_cell::sync::Lazy;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Clone, Eq)]
pub struct Atom<T: Eq + Hash + 'static>(Rc<T>);

// TODO: per-type static wouldn't work, rust
//       accepts the syntax but it's shared for all types (WTF)
//       https://github.com/rust-lang/rust/issues/22991
thread_local! {
    static ATOM_STORES: Lazy<RefCell<Vec<Box<dyn Any>>>> = Default::default();
}

impl<T: Eq + Hash + 'static> Atom<T> {
    fn with_atoms<F, R>(f: F) -> R
    where
        T: 'static,
        F: FnOnce(&mut HashSet<Rc<T>>) -> R,
    {
        ATOM_STORES.with(|stores| {
            if let Some(atoms) = stores.borrow_mut().iter_mut().find_map(|any| any.downcast_mut()) {
                return f(atoms);
            }

            let mut atoms = HashSet::new();
            let res = f(&mut atoms);

            stores.borrow_mut().push(Box::new(atoms));

            res
        })
    }
}

impl<T: 'static + Eq + Hash> From<T> for Atom<T> {
    fn from(v: T) -> Self {
        Self::with_atoms(|atoms| {
            if let Some(rc) = atoms.get(&v) {
                return Self(rc.clone());
            }

            let rc = Rc::new(v);
            atoms.insert(Rc::clone(&rc));

            Self(rc)
        })
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
        // the one which is dropped + shared dashmap
        if Rc::strong_count(&self.0) == 2 {
            Self::with_atoms(|atoms| atoms.remove(&self.0));
        }
    }
}

impl<T: Eq + Hash> Hash for Atom<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        Rc::as_ptr(&self.0).hash(hasher)
    }
}

impl<T: Eq + Hash> PartialEq for Atom<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::as_ptr(&self.0) == Rc::as_ptr(&other.0)
    }
}
