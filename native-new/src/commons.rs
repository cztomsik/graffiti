// common types & things used everywhere

/// Application unit (or something similar, unit of measure)
/// TODO(later): Integer type could save some CPU & memory
type Au = f32;

/// 2D Point
#[derive(Clone, Copy, Default, Debug)]
pub struct Pos {
    pub x: Au,
    pub y: Au
}

impl Pos {
    pub fn new(x: Au, y: Au) -> Self {
        Self { x, y }
    }

    pub fn relative_to(&self, pos: Pos) -> Pos {
        Pos { x: self.x + pos.x, y: self.y + pos.y }
    }
}

/// Bounding box defined by two points
#[derive(Clone, Copy, Default, Debug)]
pub struct Bounds {
    pub a: Pos,
    pub b: Pos
}

impl Bounds {
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
