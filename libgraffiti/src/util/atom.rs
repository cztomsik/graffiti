use fnv::FnvHashMap;
use once_cell::sync::Lazy;
use std::borrow::Borrow;
use std::fmt;
use std::num::NonZeroU32;
use std::ops::Deref;
use std::sync::RwLock;

static STORE: Lazy<RwLock<Store>> = Lazy::new(RwLock::default);

#[derive(Default)]
struct Store {
    atoms: FnvHashMap<&'static str, Atom>,
    strings: Vec<&'static str>,
}

/// Forever-interned string. Useful for identifiers and other symbols
/// which are not known in advance but the number of unique items is
/// expected to be low and/or not to grow significantly over the time.
///
/// ```
/// let atom = Atom::from("hello");
/// assert_eq!(atom, "hello");
/// assert_eq!(format!("{}", atom), "hello");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Atom(NonZeroU32);

impl<'a, B: 'a + Borrow<str>> From<B> for Atom
where
    &'a str: PartialEq<B>,
{
    fn from(v: B) -> Self {
        let Store { atoms, strings } = &mut *STORE.write().unwrap();

        if let Some(atom) = atoms.get(v.borrow()) {
            return *atom;
        }

        let s = Box::leak(Box::<str>::from(v.borrow()));
        strings.push(s);

        let atom = Self(NonZeroU32::new(strings.len() as _).unwrap());
        atoms.insert(s, atom);

        atom
    }
}

impl PartialEq<&str> for Atom {
    fn eq(&self, other: &&str) -> bool {
        *other == &**self
    }
}

impl Deref for Atom {
    type Target = str;

    fn deref(&self) -> &str {
        STORE.read().unwrap().strings[self.0.get() as usize - 1]
    }
}

impl Borrow<str> for Atom {
    fn borrow(&self) -> &str {
        &**self
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    #[test]
    fn size() {
        use std::mem::size_of;
        assert_eq!(size_of::<Option<Atom>>(), size_of::<u32>());
    }

    #[test]
    fn eq() {
        let a: Atom = Atom::from("foo");
        let b: Atom = Atom::from("foo".to_string());
        let c: Atom = Atom::from(Cow::Borrowed("foo"));

        assert_eq!(a, b);
        assert_eq!(b, c);
    }

    #[test]
    fn deref() {
        let atom = Atom::from("hello");
        assert_eq!(atom.len(), 5);
    }

    #[test]
    fn display() {
        let atom = Atom::from("hello");
        assert_eq!(format!("{}", atom), "hello");
    }
}
