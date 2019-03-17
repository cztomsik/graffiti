use crate::api::{Text};

pub trait TextMeasurer {
    fn measure_text(&self, text: &Text, avail_width: f32) -> (f32, f32);
}

pub trait TextShaper {
    // TODO: could be impl Iterator<Item = LaidGlyph> but that's not supported
    fn shape_text(&self, text: &Text, size: (f32, f32)) -> Vec<LaidGlyph>;
}

#[derive(Debug)]
pub struct LaidGlyph {
    pub glyph_index: u32,
    pub x: f32,
    pub y: f32
}

mod pango;
pub use self::pango::PangoService;
