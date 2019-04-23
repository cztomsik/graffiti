use crate::api::{SurfaceId, Rect, Size, Flex, Flow, Dimensions, Text, Overflow, Border};
use crate::text::LaidText;

/// Tree of layout nodes along with respective calculations
///
/// In future we might use `stretch` crate or maybe even something from servo
///
/// To be fast, implementation eventually has to mark "dirty" sections
/// in reaction to layout changes so it makes sense for an api to be stateful too
pub trait LayoutTree {
    fn alloc(&mut self);

    fn append_child(&mut self, parent: NodeId, child: NodeId);
    fn remove_child(&mut self, parent: NodeId, child: NodeId);
    fn insert_at(&mut self, parent: NodeId, child: NodeId, index: u32);

    fn set_size(&mut self, node_id: NodeId, size: Size);
    fn set_flex(&mut self, node_id: NodeId, flex: Flex);
    fn set_flow(&mut self, node_id: NodeId, flow: Flow);
    fn set_padding(&mut self, node_id: NodeId, padding: Dimensions);
    fn set_border(&mut self, node_id: NodeId, border: Option<Border>);
    fn set_margin(&mut self, node_id: NodeId, margin: Dimensions);
    fn set_text(&mut self, node_id: NodeId, text: Option<Text>);

    fn calculate(&mut self);
    fn computed_layout(&self, node_id: NodeId) -> Rect;
    fn text_layout(&self, node_id: NodeId) -> LaidText;
    fn set_overflow(&mut self, node_id: NodeId, overflow: Overflow);
    fn scroll_frame(&self, surface: SurfaceId) -> Option<(f32, f32)>;
}

type NodeId = SurfaceId;

mod yoga;
pub use crate::layout::yoga::YogaTree;
