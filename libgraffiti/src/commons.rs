// common types & things used everywhere

pub type ElementId = usize;

pub type TextId = usize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ElementChild {
    Element { id: ElementId },
    Text { id: TextId },
}

/// Application unit (or something similar, unit of measure)
pub type Au = f32;

/// Packed color
/// TODO: consider #[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const TRANSPARENT: Color = Self { r: 0, g: 0, b: 0, a: 0 };
    pub const BLACK: Color = Self { r: 0, g: 0, b: 0, a: 255 };

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

/// 2D Point
#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub x: Au,
    pub y: Au
}

impl Pos {
    pub const ZERO: Pos = Self { x: 0., y: 0. };

    pub fn new(x: Au, y: Au) -> Self {
        Self { x, y }
    }

    pub fn mul(&self, n: Au) -> Pos {
        Pos { x: self.x * n, y: self.y * n }
    }

    pub fn relative_to(&self, pos: Pos) -> Pos {
        Pos { x: self.x + pos.x, y: self.y + pos.y }
    }
}

/// Bounding box defined by two points
#[derive(Clone, Copy, Debug)]
pub struct Bounds {
    pub a: Pos,
    pub b: Pos
}

impl Bounds {
    pub const ZERO: Bounds = Self { a: Pos::ZERO, b: Pos::ZERO };

    pub fn mul(&self, n: Au) -> Bounds {
        let a = self.a.mul(n);
        let b = self.b.mul(n);

        Bounds { a, b }
    }

    // TODO: rename to `translate`
    pub fn relative_to(&self, pos: Pos) -> Bounds {
        let a = self.a.relative_to(pos);
        let b = self.b.relative_to(pos);

        Bounds { a, b }
    }

    pub fn contains(&self, pos: Pos) -> bool {
        pos.x > self.a.x &&
        pos.x < self.b.x &&
        pos.y > self.a.y &&
        pos.y < self.b.y
    }
}
