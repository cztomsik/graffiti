// common types & things used everywhere

use std::fmt::{self, Debug, Formatter};

/// Application unit (or something similar, unit of measure)
pub type Au = f32;

/// 2D Point
#[derive(Clone, Copy)]
pub struct Pos {
    pub x: Au,
    pub y: Au,
}

// BTW: clippy thinks it's better to pass this by value
impl Pos {
    pub const ZERO: Pos = Self { x: 0., y: 0. };
    pub const ONE: Pos = Self { x: 1., y: 1. };

    // TODO: deprecate
    #[inline]
    pub fn new(x: Au, y: Au) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn translate(self, pos: Self) -> Self {
        Self { x: self.x + pos.x, y: self.y + pos.y }
    }

    #[inline]
    pub fn mul_uniform(self, n: Au) -> Self {
        Self { x: self.x * n, y: self.y * n }
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("").field(&self.x).field(&self.y).finish()
    }
}

/// Bounding box defined by two points
#[derive(Clone, Copy)]
pub struct Bounds {
    pub a: Pos,
    pub b: Pos,
}

impl Bounds {
    pub const ZERO: Bounds = Self { a: Pos::ZERO, b: Pos::ZERO };
    pub const ZERO_ONE: Bounds = Self { a: Pos::ZERO, b: Pos::ONE };

    #[inline]
    pub fn width(&self) -> Au {
        self.b.x - self.a.x
    }

    #[inline]
    pub fn height(&self) -> Au {
        self.b.y - self.a.y
    }

    #[inline]
    pub fn inflate_uniform(&self, n: Au) -> Self {
        Self {
            a: Pos { x: self.a.x - n, y: self.a.y - n },
            b: Pos { x: self.b.x + n, y: self.b.y + n },
        }
    }

    #[inline]
    pub fn center(&self) -> Pos {
        Pos {
            x: self.a.x + (self.b.x - self.a.x) / 2.,
            y: self.a.y + (self.b.y - self.a.y) / 2.,
        }
    }

    #[inline]
    pub fn translate(&self, pos: Pos) -> Self {
        let a = self.a.translate(pos);
        let b = self.b.translate(pos);

        Self { a, b }
    }

    #[inline]
    pub fn contains(&self, pos: Pos) -> bool {
        pos.x > self.a.x && pos.x < self.b.x && pos.y > self.a.y && pos.y < self.b.y
    }
}

impl Debug for Bounds {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("Bounds").field(&self.a).field(&self.b).finish()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Mat3(pub [f32; 9]);

impl Mat3 {
    pub fn identity() -> Self {
        Self([1., 0., 0., 0., 1., 0., 0., 0., 1.])
    }
}

// couldn't use index because its result is always &V
// so it's not possible to return temp structs
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

// vecs
impl<V: Clone> Lookup<usize, V> for Vec<V> {
    fn lookup(&self, key: usize) -> V {
        self[key].clone()
    }
}

// TODO: consider UnzeroU32 so the value can be both optimized
//       with Option<T> but it also fits into 31bit V8 SMI
//       (but this is big unknown, it should be profiled first)
pub struct Id<T>(pub(crate) usize, std::marker::PhantomData<T>);

impl<T> Id<T> {
    pub(crate) const fn new(index: usize) -> Self {
        Self(index, std::marker::PhantomData)
    }
}

// can't derive https://github.com/rust-lang/rust/issues/26925
impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self(self.0, std::marker::PhantomData)
    }
}

// again
impl<T> Copy for Id<T> {}

// and again
impl<T> Debug for Id<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("").field(&self.0).finish()
    }
}

// and again
impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> std::ops::Index<Id<T>> for Vec<T> {
    type Output = T;

    fn index(&self, id: Id<T>) -> &Self::Output {
        &self[id.0]
    }
}

impl<T> std::ops::IndexMut<Id<T>> for Vec<T> {
    fn index_mut(&mut self, id: Id<T>) -> &mut Self::Output {
        &mut self[id.0]
    }
}
