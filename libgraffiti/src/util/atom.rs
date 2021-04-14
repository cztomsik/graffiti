// super-simple, unsync value interning
// TODO: NonZeroU32 (so more selector parts could be stored inline)

use std::any::Any;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{Display, Error, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Eq)]
pub struct Atom<T: Eq + Hash + 'static>(Rc<T>);

// TODO: per-type static wouldn't work, rust
//       accepts the syntax but it's shared for all types (WTF)
//       https://github.com/rust-lang/rust/issues/22991
thread_local! {
    static ATOM_STORES: RefCell<Vec<Box<dyn Any>>> = Default::default();
}

impl<T: Eq + Hash + 'static> Atom<T> {
    fn with_atoms<F, R>(f: F) -> R
    where
        T: 'static,
        F: FnOnce(&mut HashSet<Atom<T>>) -> R,
    {
        ATOM_STORES.with(|stores| {
            for any in &mut *stores.borrow_mut() {
                if let Some(atoms) = any.downcast_mut() {
                    return f(atoms);
                }
            }

            let mut atoms = HashSet::new();
            let res = f(&mut atoms);

            stores.borrow_mut().push(Box::new(atoms));

            res
        })
    }
}

impl<'a, T, Q> From<&'a Q> for Atom<T>
where
    T: 'static + Eq + Hash + Borrow<Q> + From<&'a Q>,
    Q: ?Sized + Eq + Hash,
    Atom<T>: Borrow<Q>,
{
    fn from(v: &'a Q) -> Self {
        Self::with_atoms(|atoms| {
            if let Some(atom) = atoms.get(&v) {
                return atom.clone();
            }

            let atom = Self(Rc::new(T::from(v)));
            atoms.insert(atom.clone());

            atom
        })
    }
}

// derive caused some weird type errors, I think it requires T: Clone which is wrong
impl<T: Eq + Hash> Clone for Atom<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T: Eq + Hash> Borrow<T> for Atom<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl Borrow<str> for Atom<String> {
    fn borrow(&self) -> &str {
        &self.0
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
        // the one which is dropped + shared in hashmap
        if Rc::strong_count(&self.0) == 2 {
            // take() so it's dropped when out of borrow_mut()
            Self::with_atoms(|atoms| atoms.take(self));
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

impl<T: Display + Eq + Hash> Display for Atom<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_borrowed() {
        let _: Atom<String> = Atom::from("foo");

        let foo = Atom::from("foo");
        assert!(std::any::Any::is::<Atom<String>>(&foo));
    }
}
