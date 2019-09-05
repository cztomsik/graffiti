// common types & things used everywhere

/// Application unit (or something similar, unit of measure)
/// TODO(later): Integer type could save some CPU & memory
type Au = f32;

/// 2D Point
#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub x: Au,
    pub y: Au
}

pub struct Bounds {
    pub a: Pos,
    pub b: Pos
}

impl Bounds {
    pub fn contains(&self, pos: Pos) -> bool {
        pos.0 > self.a.x
    }
}
