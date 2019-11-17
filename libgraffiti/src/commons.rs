// common types & things used everywhere

/// Application unit (or something similar, unit of measure)
/// TODO(later): Integer type could save some CPU & memory
pub type Au = f32;

/// Surfaces are everywhere
pub type SurfaceId = usize;

/// Packed color
#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn black() -> Self {
        Self { r: 0, g: 0, b: 0, a: 255 }
    }
}

/// 2D Point
#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub x: Au,
    pub y: Au
}

impl Pos {
    pub fn new(x: Au, y: Au) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0., 0.)
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
    pub fn zero() -> Self {
        Self { a: Pos::zero(), b: Pos::zero() }
    }

    pub fn mul(&self, n: Au) -> Bounds {
        let a = self.a.mul(n);
        let b = self.b.mul(n);

        Bounds { a, b }
    }

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

// not yet sure where to put these

#[derive(Debug)]
pub struct BorderRadius {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

#[derive(Debug)]
pub struct Border {
    pub top: BorderSide,
    pub right: BorderSide,
    pub bottom: BorderSide,
    pub left: BorderSide,
}

#[derive(Debug)]
pub struct BorderSide {
    pub width: f32,
    pub style: BorderStyle,
    pub color: Color,
}

#[derive(Debug)]
pub enum BorderStyle {
    None,
    Solid,
}

#[derive(Debug)]
pub struct BoxShadow {
    pub color: Color,
    pub offset: Pos,
    pub blur: f32,
    pub spread: f32,
}

#[derive(Debug)]
pub struct Image {
    pub url: String,
}
