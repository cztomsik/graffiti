pub use crate::generated::{Dimension, Flex, Rect, Size};
use crate::Id;
mod yoga;
pub use crate::layout::yoga::YogaLayoutService;

pub trait LayoutService {
    fn compute_layout(&mut self, id: Id);
}

pub type ComputedLayout = (f32, f32, f32, f32);
