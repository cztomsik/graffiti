// common types & things used everywhere

use std::fmt::{self, Debug, Formatter};

/// 2D Point
#[derive(Clone, Copy)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

// BTW: clippy thinks it's better to pass this by value
impl Pos {
    pub const ZERO: Pos = Self { x: 0., y: 0. };
    pub const ONE: Pos = Self { x: 1., y: 1. };

    // TODO: deprecate
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn translate(self, pos: Self) -> Self {
        Self { x: self.x + pos.x, y: self.y + pos.y }
    }

    #[inline]
    pub fn mul_uniform(self, n: f32) -> Self {
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
    pub fn width(&self) -> f32 {
        self.b.x - self.a.x
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.b.y - self.a.y
    }

    #[inline]
    pub fn inflate_uniform(&self, n: f32) -> Self {
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

