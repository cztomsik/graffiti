use crate::generated::SurfaceId;

/// Text layout algo
///
/// Should lay glyphs on each `Text` change without any wrapping
/// because in a lot of cases it will be enough
///
/// The box layout should call `measure_text` during its `calculate`
/// which in turn should call `wrap` if it`s needed.
pub trait TextLayout: SceneListener {
    /// Wrap/reflow existing text layout to a new max_width
    /// should skip if the `max_width` is `None` or bigger than current width
    ///
    /// Expected to be called during measure.
    /// If the `Text` is changed wrapping is reset but
    /// the box layout should again call measure which should again
    /// call the `wrap` so it should be fine (if the wrap is necessary at all)
    fn wrap(&mut self, surface: SurfaceId, max_width: Option<f32>);

    fn get_size(&self, surface: SurfaceId) -> (f32, f32);

    fn get_glyphs(&self, surface: SurfaceId) -> &[Glyph];

    // other expected use-cases (not necessarily the sole responsibility of this but related)
    // - get word boundaries at (x, y) to select word
    // - get selection boundaries from (x, y) to (x, y) during selection
    // - set cursor closest to (x, y)
    // - move cursor with keyboard arrows, respecting wrapping
    // - select next word
    
}

#[derive(Debug)]
pub struct Glyph {
    pub x: f32,
    pub y: f32,
    pub glyph_id: u32
}

mod simple;
pub use self::simple::SimpleTextLayout;
use crate::SceneListener;
