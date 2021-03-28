// x easy to test
// x return (impl-specific) handles
// x keep & organize layout nodes
// x set all props at once
// x calculate & provide box bounds for rendering
// x bounds relative to their parents
// x node/leaf type cannot be changed (but you can always create a new one and replace it)

use graffiti_yoga::*;
use std::convert::TryInto;

pub struct LayoutEngine {}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_node(&mut self) -> LayoutNode {
        unsafe {
            let node = YGNodeNew();

            YGNodeStyleSetPadding(node, YGEdge::All, 10.);
            YGNodeStyleSetMinWidth(node, 100.);
            YGNodeStyleSetMinHeight(node, 20.);

            LayoutNode(node)
        }
    }

    pub fn insert_child(&mut self, parent: LayoutNode, child: LayoutNode, index: usize) {
        unsafe { YGNodeInsertChild(parent.0, child.0, index.try_into().unwrap()) }
    }

    pub fn remove_child(&mut self, parent: LayoutNode, child: LayoutNode) {
        unsafe { YGNodeRemoveChild(parent.0, child.0) }
    }

    pub fn calculate(&mut self, root: LayoutNode, avail_size: (f32, f32)) {
        // height is ignored (yoga would use it as maxHeight which is not what we want, for now)
        unsafe { YGNodeCalculateLayout(root.0, avail_size.0, YGUndefined, YGDirection::LTR) }
    }

    #[inline]
    pub fn node_offset(&self, node: LayoutNode) -> (f32, f32) {
        unsafe { (YGNodeLayoutGetLeft(node.0), YGNodeLayoutGetTop(node.0)) }
    }

    #[inline]
    pub fn node_size(&self, node: LayoutNode) -> (f32, f32) {
        unsafe { (YGNodeLayoutGetWidth(node.0), YGNodeLayoutGetHeight(node.0)) }
    }

    pub fn drop_node(&mut self, node: LayoutNode) {
        unsafe { YGNodeFree(node.0) }
    }
}

#[derive(Clone, Copy)]
pub struct LayoutNode(YGNodeRef);

unsafe impl Send for LayoutNode {}
unsafe impl Sync for LayoutNode {}
