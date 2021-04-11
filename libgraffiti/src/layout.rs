// x independent, easy to test
// x return (impl-specific) handles
// x keep & organize layout nodes
// x set props one by one
// x calculate & provide box bounds for rendering
// x bounds relative to their parents
// x node/leaf type cannot be changed (but you can always create a new one and replace it)

use graffiti_yoga::*;
use std::convert::TryInto;

pub type Display = YGDisplay;
pub type FlexDirection = YGFlexDirection;
pub type FlexWrap = YGWrap;
pub type Align = YGAlign;
pub type Justify = YGJustify;
pub type Position = YGPositionType;

pub enum Dimension {
    Undefined,
    Px(f32),
    Percent(f32),
    Auto,
}

pub struct LayoutEngine;

macro_rules! set_dim {
    ($node:expr, $value:expr; $set:ident $set_perc:ident $($set_auto:ident)*) => (
        unsafe {
            match $value {
                Dimension::Px(v) => $set($node.0, v),
                Dimension::Percent(v) => $set_perc($node.0, v),
                $(Dimension::Auto => $set_auto($node.0),)*
                _ => $set($node.0, YGUndefined)
            }
        }
    )
}

macro_rules! set_edge_dim {
    ($node:expr, $edge:expr, $value:expr; $set:ident $set_perc:ident $($set_auto:ident)*) => (
        unsafe {
            match $value {
                Dimension::Px(v) => $set($node.0, $edge, v),
                Dimension::Percent(v) => $set_perc($node.0, $edge, v),
                $(Dimension::Auto => $set_auto($node.0, $edge),)*
                _ => $set($node.0, $edge, YGUndefined)
            }
        }
    )
}

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

    pub fn create_node(&mut self) -> LayoutNode {
        let node = LayoutNode(unsafe { YGNodeNew() });

        // TODO: default should be "inline"

        // set web defaults
        //self.set_flex_direction(node, FlexDirection::Row);
        //self.set_flex_basis(node, Dimension::Auto);
        //self.set_flex_shrink(node, 1.);

        node
    }

    pub fn insert_child(&mut self, parent: LayoutNode, child: LayoutNode, index: usize) {
        unsafe { YGNodeInsertChild(parent.0, child.0, index.try_into().unwrap()) }
    }

    pub fn remove_child(&mut self, parent: LayoutNode, child: LayoutNode) {
        unsafe { YGNodeRemoveChild(parent.0, child.0) }
    }

    // TODO: getters

    pub fn display(&self, node: LayoutNode) -> Display {
        todo!()
    }

    pub fn set_display(&mut self, node: LayoutNode, value: Display) {
        unsafe { YGNodeStyleSetDisplay(node.0, value) }
    }

    pub fn width(&self, node: LayoutNode) -> Dimension {
        todo!()
    }

    pub fn set_width(&mut self, node: LayoutNode, value: Dimension) {
        set_dim!(node, value; YGNodeStyleSetWidth YGNodeStyleSetWidthPercent YGNodeStyleSetWidthAuto);
    }

    pub fn height(&self, node: LayoutNode) -> Dimension {
        todo!()
    }

    pub fn set_height(&mut self, node: LayoutNode, value: Dimension) {
        set_dim!(node, value; YGNodeStyleSetHeight YGNodeStyleSetHeightPercent YGNodeStyleSetHeightAuto);
    }

    pub fn min_width(&self, node: LayoutNode) -> Dimension {
        todo!()
    }

    pub fn set_min_width(&mut self, node: LayoutNode, value: Dimension) {
        set_dim!(node, value; YGNodeStyleSetMinWidth YGNodeStyleSetWidthPercent);
    }

    pub fn min_height(&self, node: LayoutNode) -> Dimension {
        todo!()
    }

    pub fn set_min_height(&mut self, node: LayoutNode, value: Dimension) {
        set_dim!(node, value; YGNodeStyleSetMinHeight YGNodeStyleSetHeightPercent);
    }

    pub fn max_width(&self, node: LayoutNode) -> Dimension {
        todo!()
    }

    pub fn set_max_width(&mut self, node: LayoutNode, value: Dimension) {
        set_dim!(node, value; YGNodeStyleSetMaxWidth YGNodeStyleSetMaxWidthPercent);
    }

    pub fn max_height(&self, node: LayoutNode) -> Dimension {
        todo!()
    }

    pub fn set_max_height(&mut self, node: LayoutNode, value: Dimension) {
        set_dim!(node, value; YGNodeStyleSetMaxHeight YGNodeStyleSetMaxHeightPercent);
    }

    pub fn top(&self, node: LayoutNode) -> Dimension {
        todo!()
    }

    pub fn set_top(&mut self, node: LayoutNode, value: Dimension) {
        set_edge_dim!(node, YGEdge::Top, value; YGNodeStyleSetPosition YGNodeStyleSetPositionPercent);
    }

    pub fn right(&self, node: LayoutNode) -> Dimension {
        todo!()
    }

    pub fn set_right(&mut self, node: LayoutNode, value: Dimension) {
        set_edge_dim!(node, YGEdge::Right, value; YGNodeStyleSetPosition YGNodeStyleSetPositionPercent);
    }

    pub fn bottom(&self, node: LayoutNode) -> Dimension {
        todo!()
    }

    pub fn set_bottom(&mut self, node: LayoutNode, value: Dimension) {
        set_edge_dim!(node, YGEdge::Bottom, value; YGNodeStyleSetPosition YGNodeStyleSetPositionPercent);
    }

    pub fn left(&self, node: LayoutNode) -> Dimension {
        todo!()
    }

    pub fn set_left(&mut self, node: LayoutNode, value: Dimension) {
        set_edge_dim!(node, YGEdge::Left, value; YGNodeStyleSetPosition YGNodeStyleSetPositionPercent);
    }

    pub fn margin_top(&self, node: LayoutNode) -> Dimension {
        todo!()
    }

    pub fn set_margin_top(&mut self, node: LayoutNode, value: Dimension) {
        set_edge_dim!(node, YGEdge::Top, value; YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto);
    }

    pub fn margin_right(&self, node: LayoutNode) -> Dimension {
        todo!()
    }

    pub fn set_margin_right(&mut self, node: LayoutNode, value: Dimension) {
        set_edge_dim!(node, YGEdge::Right, value; YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto);
    }

    pub fn margin_bottom(&self, node: LayoutNode) -> Dimension {
        todo!()
    }

    pub fn set_margin_bottom(&mut self, node: LayoutNode, value: Dimension) {
        set_edge_dim!(node, YGEdge::Bottom, value; YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto);
    }

    pub fn margin_left(&self, node: LayoutNode) -> Dimension {
        todo!()
    }

    pub fn set_margin_left(&mut self, node: LayoutNode, value: Dimension) {
        set_edge_dim!(node, YGEdge::Left, value; YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto);
    }

    pub fn padding_top(&self, node: LayoutNode) -> Dimension {
        todo!()
    }

    pub fn set_padding_top(&mut self, node: LayoutNode, value: Dimension) {
        set_edge_dim!(node, YGEdge::Top, value; YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent);
    }

    pub fn padding_right(&self, node: LayoutNode) -> Dimension {
        todo!()
    }

    pub fn set_padding_right(&mut self, node: LayoutNode, value: Dimension) {
        set_edge_dim!(node, YGEdge::Right, value; YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent);
    }

    pub fn padding_bottom(&self, node: LayoutNode) -> Dimension {
        todo!()
    }

    pub fn set_padding_bottom(&mut self, node: LayoutNode, value: Dimension) {
        set_edge_dim!(node, YGEdge::Bottom, value; YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent);
    }

    pub fn padding_left(&self, node: LayoutNode) -> Dimension {
        todo!()
    }

    pub fn set_padding_left(&mut self, node: LayoutNode, value: Dimension) {
        set_edge_dim!(node, YGEdge::Left, value; YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent);
    }

    pub fn border_top(&self, node: LayoutNode) -> f32 {
        todo!()
    }

    pub fn set_border_top(&mut self, node: LayoutNode, value: f32) {
        unsafe { YGNodeStyleSetBorder(node.0, YGEdge::Top, value) }
    }

    pub fn border_right(&self, node: LayoutNode) -> f32 {
        todo!()
    }

    pub fn set_border_right(&mut self, node: LayoutNode, value: f32) {
        unsafe { YGNodeStyleSetBorder(node.0, YGEdge::Right, value) }
    }

    pub fn border_bottom(&self, node: LayoutNode) -> f32 {
        todo!()
    }

    pub fn set_border_bottom(&mut self, node: LayoutNode, value: f32) {
        unsafe { YGNodeStyleSetBorder(node.0, YGEdge::Bottom, value) }
    }

    pub fn border_left(&self, node: LayoutNode) -> f32 {
        todo!()
    }

    pub fn set_border_left(&mut self, node: LayoutNode, value: f32) {
        unsafe { YGNodeStyleSetBorder(node.0, YGEdge::Left, value) }
    }

    pub fn flex_grow(&self, node: LayoutNode) -> f32 {
        todo!()
    }

    pub fn set_flex_grow(&mut self, node: LayoutNode, value: f32) {
        unsafe { YGNodeStyleSetFlexGrow(node.0, value) }
    }

    pub fn flex_shrink(&self, node: LayoutNode) -> f32 {
        todo!()
    }

    pub fn set_flex_shrink(&mut self, node: LayoutNode, value: f32) {
        unsafe { YGNodeStyleSetFlexShrink(node.0, value) }
    }

    pub fn flex_basis(&self, node: LayoutNode) -> Dimension {
        todo!()
    }

    pub fn set_flex_basis(&mut self, node: LayoutNode, value: Dimension) {
        set_dim!(node, value; YGNodeStyleSetFlexBasis YGNodeStyleSetFlexBasisPercent YGNodeStyleSetFlexBasisAuto);
    }

    pub fn flex_direction(&mut self, node: LayoutNode) -> FlexDirection {
        todo!()
    }

    pub fn set_flex_direction(&mut self, node: LayoutNode, value: FlexDirection) {
        unsafe { YGNodeStyleSetFlexDirection(node.0, value) }
    }

    pub fn flex_wrap(&mut self, node: LayoutNode) -> FlexWrap {
        todo!()
    }

    pub fn set_flex_wrap(&mut self, node: LayoutNode, value: FlexWrap) {
        unsafe { YGNodeStyleSetFlexWrap(node.0, value) }
    }

    pub fn align_self(&mut self, node: LayoutNode) -> Align {
        todo!()
    }

    pub fn set_align_self(&mut self, node: LayoutNode, value: Align) {
        unsafe { YGNodeStyleSetAlignSelf(node.0, value) }
    }

    pub fn align_content(&mut self, node: LayoutNode) -> Align {
        todo!()
    }

    pub fn set_align_content(&mut self, node: LayoutNode, value: Align) {
        unsafe { YGNodeStyleSetAlignContent(node.0, value) }
    }

    pub fn align_items(&mut self, node: LayoutNode) -> Align {
        todo!()
    }

    pub fn set_align_items(&mut self, node: LayoutNode, value: Align) {
        unsafe { YGNodeStyleSetAlignItems(node.0, value) }
    }

    pub fn justify_content(&mut self, node: LayoutNode) -> Justify {
        todo!()
    }

    pub fn set_justify_content(&mut self, node: LayoutNode, value: Justify) {
        unsafe { YGNodeStyleSetJustifyContent(node.0, value) }
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
