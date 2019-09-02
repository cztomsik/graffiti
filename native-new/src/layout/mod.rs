use crate::SceneListener;
use crate::generated::{SurfaceId, Rect};

/// Box layout algo
/// Only flexbox is expected for now, grid might be added in the future
///
/// The text layout is a separate thing and the only relation is that
/// the box layout (sometimes) needs to measure the text content to determine box sizes.
/// For this purpose the `measure_text` callback is provided to the `calculate` method.
pub trait Layout: SceneListener {
  fn calculate(&mut self, measure_text: &mut dyn FnMut(SurfaceId, Option<f32>) -> (f32, f32));

  fn get_rect(&self, surface: SurfaceId) -> Rect;

  fn get_scroll_frame(&self, surface: SurfaceId) -> Option<(f32, f32)>;
}

// mod yoga;
// pub use crate::layout::yoga::YogaLayout;

mod stretch;
pub use crate::layout::stretch::StretchLayout;
