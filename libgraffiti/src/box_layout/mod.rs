use crate::generated::{SurfaceId, UpdateSceneMsg};
use crate::commons::Bounds;

/// Box layout algo
/// Only flexbox is expected for now, grid might be added in the future
///
/// The text layout is a separate thing and the only relation is that
/// the box layout (sometimes) needs to measure the text content to determine box sizes.
/// For this purpose the `measure_text` callback is provided to the `calculate` method.
pub trait BoxLayout {
  fn update_scene(&mut self, msgs: &[UpdateSceneMsg]);

  fn calculate(&mut self, measure_text: &mut dyn FnMut(SurfaceId, Option<f32>) -> (f32, f32));

  // TODO: not sure if it's necessary for the picker but for rendering
  // we could be fine with <T: Index<SurfaceId>> because the bounds
  // are looked up only once for each surface context so technically,
  // it doesn't have to be continuous slice in memory
  fn get_bounds(&self) -> &[Bounds];
}

mod yoga;
pub use crate::box_layout::yoga::YogaLayout;

// mod stretch;
// pub use self::stretch::StretchLayout;
