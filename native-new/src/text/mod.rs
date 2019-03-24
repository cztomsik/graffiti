use crate::api::{Text};

pub trait TextMeasurer {
    fn measure_text(&self, text: &Text, max_width: Option<f32>) -> (f32, f32);
}

pub trait TextShaper {
    // TODO: could be impl Iterator<Item = LaidGlyph> but that's not supported
    fn shape_text(&self, text: &Text, size: (f32, f32)) -> Vec<LaidGlyph>;
}

pub struct LaidGlyph {
    pub glyph_index: u32,
    pub x: f32,
    pub y: f32
}

impl Debug for LaidGlyph {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?} ", (self.glyph_index, self.x, self.y))
    }
}
mod pango;
pub use self::pango::PangoService;

use std::fmt::{Debug, Formatter};
