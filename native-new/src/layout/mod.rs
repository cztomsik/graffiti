mod yoga;
pub use crate::layout::yoga::YogaLayoutService;
use crate::api::{SurfaceId, ComputedLayout};

pub trait LayoutService {
    fn get_computed_layouts(&mut self, surface: SurfaceId) -> Vec<ComputedLayout>;
}
