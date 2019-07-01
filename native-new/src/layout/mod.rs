use crate::SceneListener;
use crate::generated::{SurfaceId, Rect, Vector2f};

pub trait Layout: SceneListener {
  fn calculate(&mut self, measure_text: &mut dyn FnMut(SurfaceId, Option<f32>) -> Vector2f);

  fn get_rect(&self, surface: SurfaceId) -> Rect;
}

mod yoga;
pub use crate::layout::yoga::YogaLayout;
