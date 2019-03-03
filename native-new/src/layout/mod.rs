pub use crate::generated::{Dimension, Flex, Rect, Size};
mod yoga;
pub use crate::layout::yoga::YogaLayoutService;
use crate::surface::SurfaceData;

pub trait LayoutService {
    fn get_computed_layouts(&mut self, surface: &SurfaceData) -> Vec<ComputedLayout>;
}

pub type ComputedLayout = (f32, f32, f32, f32);
