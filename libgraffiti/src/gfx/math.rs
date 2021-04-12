// mostly resurrected from previous codebase

use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self::new(0., 0.);
    pub const ONE: Self = Self::new(1., 1.);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, n: f32) -> Self {
        Self {
            x: self.x * n,
            y: self.y * n,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, n: f32) -> Self {
        self * (1. / n)
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    pub const ZERO: Self = Self::new(Vec2::ZERO, Vec2::ZERO);

    pub const fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    pub fn center(&self) -> Vec2 {
        self.min + (self.max - self.min) / 2.
    }

    pub fn contains(&self, pos: Vec2) -> bool {
        pos.x > self.min.x && pos.x < self.max.x && pos.y > self.min.y && pos.y < self.max.y
    }
}

impl Mul<f32> for AABB {
    type Output = Self;

    fn mul(self, n: f32) -> Self {
        Self {
            min: self.min * n,
            max: self.max * n,
        }
    }
}

impl Add<Vec2> for AABB {
    type Output = Self;

    fn add(self, pos: Vec2) -> Self {
        Self {
            min: self.min + pos,
            max: self.max + pos,
        }
    }
}
