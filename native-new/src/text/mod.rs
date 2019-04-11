use crate::api::{Text};

/// Measuring & laying the glyphs might seem a bit orthogonal but to measure we need to know
/// a lot of what is then also needed for glyph positioning so it makes sense to do it just once.
///
/// It might have a state, but it's rather global, there should be no dependencies on given window,
/// nor scene/surfaces.
///
/// Although we could return stateful TextLayouts and make some successive updates a bit faster,
/// I think it's not worth and it's better to focus on other things.
pub trait TextLayoutAlgo {
    // TODO: could be impl Iterator<Item = LaidGlyph> but that's not supported
    fn layout_text(&mut self, text: &Text, max_width: Option<f32>) -> LaidText;
}

// TODO: generate (so it can be sent to js)
#[derive(Clone, Debug)]
pub struct LaidText {
    pub lines: i32,
    pub width: f32,
    pub glyphs: Vec<LaidGlyph>
}

#[derive(Clone)]
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
