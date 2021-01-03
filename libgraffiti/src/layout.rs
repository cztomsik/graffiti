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

            node
        }
    }

    pub fn insert_child(&mut self, parent: LayoutNode, child: LayoutNode, index: usize) {
        unsafe { YGNodeInsertChild(parent, child, index.try_into().unwrap()) }
    }

    pub fn remove_child(&mut self, parent: LayoutNode, child: LayoutNode) {
        unsafe { YGNodeRemoveChild(parent, child) }
    }

    pub fn calculate(&mut self, root: LayoutNode, avail_size: (f32, f32)) {
        // height is ignored (yoga would use it as maxHeight which is not what we want, for now)
        unsafe { YGNodeCalculateLayout(root, avail_size.0, YGUndefined, YGDirection::LTR) }
    }

    #[inline]
    pub fn node_offset(&self, node: YGNodeRef) -> (f32, f32) {
        unsafe { (YGNodeLayoutGetLeft(node), YGNodeLayoutGetTop(node)) }
    }

    #[inline]
    pub fn node_size(&self, node: YGNodeRef) -> (f32, f32) {
        unsafe { (YGNodeLayoutGetWidth(node), YGNodeLayoutGetHeight(node)) }
    }

    pub fn free_node(&mut self, node: LayoutNode) {
        unsafe { YGNodeFree(node) }
    }
}

pub type LayoutNode = YGNodeRef;
