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

    pub fn create_leaf<F: Fn(f32) -> (f32, f32)>(&mut self, measure: F) -> LayoutNode {
        unsafe {
            let node = YGNodeNew();

            YGNodeSetMeasureFunc(node, Some(measure_node::<F>));
            // TODO: drop
            YGNodeSetContext(node, Box::into_raw(Box::new(measure)) as _);

            LayoutNode(node)
        }
    }

    pub fn mark_dirty(&mut self, node: LayoutNode) {
        unsafe { YGNodeMarkDirty(node.0) }
    }

    pub fn create_node(&mut self, style: &LayoutStyle) -> LayoutNode {
        let node = LayoutNode(unsafe { YGNodeNew() });
        self.set_style(node, style);

        node
    }

    pub fn set_style(&mut self, node: LayoutNode, style: &LayoutStyle) {
        unsafe {
            YGNodeStyleSetDisplay(node.0, style.display);

            // TODO
            YGNodeStyleSetFlexDirection(node.0, YGFlexDirection::Row);
            YGNodeStyleSetPadding(node.0, YGEdge::All, 10.);
            YGNodeStyleSetMinWidth(node.0, 100.);
            YGNodeStyleSetMinHeight(node.0, 20.);
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

// TODO: (xor)diff props or masks
pub struct LayoutStyle {
    display: YGDisplay,
}

impl LayoutStyle {
    pub const DEFAULT: Self = Self {
        display: YGDisplay::Flex,
    };

    pub const HIDDEN: Self = Self {
        display: YGDisplay::None,
        ..Self::DEFAULT
    };
}

#[derive(Debug, Clone, Copy)]
pub struct LayoutNode(YGNodeRef);

unsafe impl Send for LayoutNode {}
unsafe impl Sync for LayoutNode {}

unsafe extern "C" fn measure_node<F: Fn(f32) -> (f32, f32)>(
    node: YGNodeRef,
    w: f32,
    wm: YGMeasureMode,
    _h: f32,
    _hm: YGMeasureMode,
) -> YGSize {
    let max_width = match wm {
        YGMeasureMode::Exactly => w,
        YGMeasureMode::AtMost => w,
        YGMeasureMode::Undefined => std::f32::MAX,
    };

    let measure: *mut F = YGNodeGetContext(node) as _;
    let size = (*measure)(max_width);

    YGSize {
        width: match wm {
            YGMeasureMode::Exactly => w,
            _ => size.0,
        },
        height: size.1,
    }
}
