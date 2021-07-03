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
pub struct Atom<T: Eq + Hash + 'static>(Wrap<T>);

// pub because of trait bounds for From<>
#[doc(hidden)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Wrap<T>(Rc<T>);

// TODO: per-type static wouldn't work, rust
//       accepts the syntax but it's shared for all types (WTF)
//       https://github.com/rust-lang/rust/issues/22991
thread_local! {
    static ATOM_STORES: RefCell<Vec<Box<dyn Any>>> = Default::default();
}

impl<T: Eq + Hash + 'static> Atom<T> {
    fn with_wraps<F, R>(f: F) -> R
    where
        T: 'static,
        F: FnOnce(&mut HashSet<Wrap<T>>) -> R,
    {
        ATOM_STORES.with(|stores| {
            for any in &mut *stores.borrow_mut() {
                if let Some(wraps) = any.downcast_mut() {
                    return f(wraps);
                }
            }
            let mut wraps = HashSet::new();
            let res = f(&mut wraps);
            stores.borrow_mut().push(Box::new(wraps));
            res
        })
    }
}

impl<'a, T, Q> From<&'a Q> for Atom<T>
where
    T: 'static + Eq + Hash + Borrow<Q> + From<&'a Q>,
    Q: ?Sized + Eq + Hash,
    Wrap<T>: Borrow<Q>,
{
    fn from(v: &'a Q) -> Self {
        Self::with_wraps(|wraps| {
            // beware that Hash for Atom<> is different than the one here
            if let Some(wrap) = wraps.get(&v) {
                return Self(Wrap(wrap.0.clone()));
            }
            let wrap = Wrap(Rc::new(T::from(v)));
            wraps.insert(Wrap(wrap.0.clone()));
            Self(wrap)
        })
    }
}

// derive caused some weird type errors, I think it requires T: Clone which is wrong
impl<T: Eq + Hash> Clone for Atom<T> {
    fn clone(&self) -> Self {
        Self(Wrap(Rc::clone(&self.0 .0)))
    }
}

impl<T: Eq + Hash> Borrow<T> for Wrap<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl Borrow<str> for Wrap<String> {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl<T: Eq + Hash> Deref for Atom<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.0 .0.as_ref()
    }
}

impl<T: Eq + Hash + 'static> Drop for Atom<T> {
    fn drop(&mut self) {
        // the one which is dropped + one shared in hashset
        if Rc::strong_count(&self.0 .0) == 2 {
            Self::with_wraps(|wraps| wraps.remove(&self.0));
        }
    }
}

impl<T: Eq + Hash> Hash for Atom<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        Rc::as_ptr(&self.0 .0).hash(hasher)
    }
}

impl<T: Eq + Hash> PartialEq for Atom<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::as_ptr(&self.0 .0) == Rc::as_ptr(&other.0 .0)
    }
}

impl<T: Display + Eq + Hash> Display for Atom<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.0 .0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq() {
        let a: Atom<String> = Atom::from("foo");
        let b: Atom<String> = Atom::from(&"foo".to_owned());

        assert_eq!(a, b);
        assert_eq!(a.0 .0.as_ptr(), b.0 .0.as_ptr());
    }

    #[test]
    fn from_borrowed() {
        let _: Atom<String> = Atom::from("foo");

        let foo = Atom::from("foo");
        assert!(<dyn std::any::Any>::is::<Atom<String>>(&foo));
    }
}
