use crate::SceneListener;
use crate::generated::{SurfaceId, Rect};

pub trait Layout: SceneListener {
  fn get_rect(&self, surface: SurfaceId) -> Rect;
}

mod yoga;
pub use crate::layout::yoga::YogaLayout;
