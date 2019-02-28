pub use crate::generated::{Flex, Rect, Size, Dimension};
use crate::Id;
mod yoga;
pub use crate::layout::yoga::YogaLayoutService;

pub trait LayoutService {
    fn append_child(&mut self, parent: Id, child: Id);
    fn remove_child(&mut self, parent: Id, child: Id);

    // easier with index rather than with Id
    fn insert_at(&mut self, parent: Id, child: Id, index: u32);

    fn set_size(&mut self, id: Id, size: Size);
    fn set_flex(&mut self, id: Id, flex: Flex);
    fn set_padding(&mut self, id: Id, padding: Rect);
    fn set_margin(&mut self, id: Id, padding: Rect);
    fn compute_layout(&mut self, id: Id);
    fn get_computed_layout(&self, id: Id) -> ComputedLayout;
}

pub type ComputedLayout = (f32, f32, f32, f32);
