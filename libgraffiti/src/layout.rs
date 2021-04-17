// x independent, easy to test
// x set props one by one
// x calculate & provide box bounds for rendering
// x bounds relative to their parents
// x node/leaf type cannot be changed

#![allow(unused)]

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

#[derive(Debug)]
pub struct LayoutNode(YGNodeRef);

impl LayoutNode {
    pub fn new() -> Self {
        let node = Self(unsafe { YGNodeNew() });

        // TODO: default should be "inline"

        // set web defaults
        node.set_flex_direction(FlexDirection::Row);
        node.set_align_content(Align::Stretch);
        //node.set_flex_basis(Dimension::Auto);
        //node.set_flex_shrink(1.);

        node
    }

    pub fn new_leaf<F: Fn(f32) -> (f32, f32)>(measure: F) -> Self {
        unsafe {
            let node = YGNodeNew();

            YGNodeSetMeasureFunc(node, Some(measure_node::<F>));
            // TODO: drop
            YGNodeSetContext(node, Box::into_raw(Box::new(measure)) as _);

            Self(node)
        }
    }

    pub fn mark_dirty(&self) {
        unsafe { YGNodeMarkDirty(self.0) }
    }

    pub fn insert_child(&self, child: &LayoutNode, index: usize) {
        unsafe { YGNodeInsertChild(self.0, child.0, index.try_into().unwrap()) }
    }

    pub fn remove_child(&self, child: &LayoutNode) {
        unsafe { YGNodeRemoveChild(self.0, child.0) }
    }

    // TODO: getters

    pub fn display(&self) -> Display {
        todo!()
    }

    pub fn set_display(&self, value: Display) {
        unsafe { YGNodeStyleSetDisplay(self.0, value) }
    }

    pub fn width(&self) -> Dimension {
        todo!()
    }

    pub fn set_width(&self, value: Dimension) {
        set_dim!(self, value; YGNodeStyleSetWidth YGNodeStyleSetWidthPercent YGNodeStyleSetWidthAuto);
    }

    pub fn height(&self) -> Dimension {
        todo!()
    }

    pub fn set_height(&self, value: Dimension) {
        set_dim!(self, value; YGNodeStyleSetHeight YGNodeStyleSetHeightPercent YGNodeStyleSetHeightAuto);
    }

    pub fn min_width(&self) -> Dimension {
        todo!()
    }

    pub fn set_min_width(&self, value: Dimension) {
        set_dim!(self, value; YGNodeStyleSetMinWidth YGNodeStyleSetWidthPercent);
    }

    pub fn min_height(&self) -> Dimension {
        todo!()
    }

    pub fn set_min_height(&self, value: Dimension) {
        set_dim!(self, value; YGNodeStyleSetMinHeight YGNodeStyleSetHeightPercent);
    }

    pub fn max_width(&self) -> Dimension {
        todo!()
    }

    pub fn set_max_width(&self, value: Dimension) {
        set_dim!(self, value; YGNodeStyleSetMaxWidth YGNodeStyleSetMaxWidthPercent);
    }

    pub fn max_height(&self) -> Dimension {
        todo!()
    }

    pub fn set_max_height(&self, value: Dimension) {
        set_dim!(self, value; YGNodeStyleSetMaxHeight YGNodeStyleSetMaxHeightPercent);
    }

    pub fn position(&self) -> Position {
        todo!()
    }

    pub fn set_position(&self, value: Position) {
        unsafe { YGNodeStyleSetPositionType(self.0, value) }
    }

    pub fn top(&self) -> Dimension {
        todo!()
    }

    pub fn set_top(&self, value: Dimension) {
        set_edge_dim!(self, YGEdge::Top, value; YGNodeStyleSetPosition YGNodeStyleSetPositionPercent);
    }

    pub fn right(&self) -> Dimension {
        todo!()
    }

    pub fn set_right(&self, value: Dimension) {
        set_edge_dim!(self, YGEdge::Right, value; YGNodeStyleSetPosition YGNodeStyleSetPositionPercent);
    }

    pub fn bottom(&self) -> Dimension {
        todo!()
    }

    pub fn set_bottom(&self, value: Dimension) {
        set_edge_dim!(self, YGEdge::Bottom, value; YGNodeStyleSetPosition YGNodeStyleSetPositionPercent);
    }

    pub fn left(&self) -> Dimension {
        todo!()
    }

    pub fn set_left(&self, value: Dimension) {
        set_edge_dim!(self, YGEdge::Left, value; YGNodeStyleSetPosition YGNodeStyleSetPositionPercent);
    }

    pub fn margin_top(&self) -> Dimension {
        todo!()
    }

    pub fn set_margin_top(&self, value: Dimension) {
        set_edge_dim!(self, YGEdge::Top, value; YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto);
    }

    pub fn margin_right(&self) -> Dimension {
        todo!()
    }

    pub fn set_margin_right(&self, value: Dimension) {
        set_edge_dim!(self, YGEdge::Right, value; YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto);
    }

    pub fn margin_bottom(&self) -> Dimension {
        todo!()
    }

    pub fn set_margin_bottom(&self, value: Dimension) {
        set_edge_dim!(self, YGEdge::Bottom, value; YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto);
    }

    pub fn margin_left(&self) -> Dimension {
        todo!()
    }

    pub fn set_margin_left(&self, value: Dimension) {
        set_edge_dim!(self, YGEdge::Left, value; YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto);
    }

    pub fn padding_top(&self) -> Dimension {
        todo!()
    }

    pub fn set_padding_top(&self, value: Dimension) {
        set_edge_dim!(self, YGEdge::Top, value; YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent);
    }

    pub fn padding_right(&self) -> Dimension {
        todo!()
    }

    pub fn set_padding_right(&self, value: Dimension) {
        set_edge_dim!(self, YGEdge::Right, value; YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent);
    }

    pub fn padding_bottom(&self) -> Dimension {
        todo!()
    }

    pub fn set_padding_bottom(&self, value: Dimension) {
        set_edge_dim!(self, YGEdge::Bottom, value; YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent);
    }

    pub fn padding_left(&self) -> Dimension {
        todo!()
    }

    pub fn set_padding_left(&self, value: Dimension) {
        set_edge_dim!(self, YGEdge::Left, value; YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent);
    }

    pub fn border_top(&self) -> f32 {
        todo!()
    }

    pub fn set_border_top(&self, value: f32) {
        unsafe { YGNodeStyleSetBorder(self.0, YGEdge::Top, value) }
    }

    pub fn border_right(&self) -> f32 {
        todo!()
    }

    pub fn set_border_right(&self, value: f32) {
        unsafe { YGNodeStyleSetBorder(self.0, YGEdge::Right, value) }
    }

    pub fn border_bottom(&self) -> f32 {
        todo!()
    }

    pub fn set_border_bottom(&self, value: f32) {
        unsafe { YGNodeStyleSetBorder(self.0, YGEdge::Bottom, value) }
    }

    pub fn border_left(&self) -> f32 {
        todo!()
    }

    pub fn set_border_left(&self, value: f32) {
        unsafe { YGNodeStyleSetBorder(self.0, YGEdge::Left, value) }
    }

    pub fn flex_grow(&self) -> f32 {
        todo!()
    }

    pub fn set_flex_grow(&self, value: f32) {
        unsafe { YGNodeStyleSetFlexGrow(self.0, value) }
    }

    pub fn flex_shrink(&self) -> f32 {
        todo!()
    }

    pub fn set_flex_shrink(&self, value: f32) {
        unsafe { YGNodeStyleSetFlexShrink(self.0, value) }
    }

    pub fn flex_basis(&self) -> Dimension {
        todo!()
    }

    pub fn set_flex_basis(&self, value: Dimension) {
        set_dim!(self, value; YGNodeStyleSetFlexBasis YGNodeStyleSetFlexBasisPercent YGNodeStyleSetFlexBasisAuto);
    }

    pub fn flex_direction(&self) -> FlexDirection {
        todo!()
    }

    pub fn set_flex_direction(&self, value: FlexDirection) {
        unsafe { YGNodeStyleSetFlexDirection(self.0, value) }
    }

    pub fn flex_wrap(&self) -> FlexWrap {
        todo!()
    }

    pub fn set_flex_wrap(&self, value: FlexWrap) {
        unsafe { YGNodeStyleSetFlexWrap(self.0, value) }
    }

    pub fn align_self(&self) -> Align {
        todo!()
    }

    pub fn set_align_self(&self, value: Align) {
        unsafe { YGNodeStyleSetAlignSelf(self.0, value) }
    }

    pub fn align_content(&self) -> Align {
        todo!()
    }

    pub fn set_align_content(&self, value: Align) {
        unsafe { YGNodeStyleSetAlignContent(self.0, value) }
    }

    pub fn align_items(&self) -> Align {
        todo!()
    }

    pub fn set_align_items(&self, value: Align) {
        unsafe { YGNodeStyleSetAlignItems(self.0, value) }
    }

    pub fn justify_content(&self) -> Justify {
        todo!()
    }

    pub fn set_justify_content(&self, value: Justify) {
        unsafe { YGNodeStyleSetJustifyContent(self.0, value) }
    }

    pub fn calculate(&self, avail_size: (f32, f32)) {
        unsafe { YGNodeCalculateLayout(self.0, avail_size.0, avail_size.1, YGDirection::LTR) }
    }

    #[inline]
    pub fn offset(&self) -> (f32, f32) {
        unsafe { (YGNodeLayoutGetLeft(self.0), YGNodeLayoutGetTop(self.0)) }
    }

    #[inline]
    pub fn size(&self) -> (f32, f32) {
        unsafe { (YGNodeLayoutGetWidth(self.0), YGNodeLayoutGetHeight(self.0)) }
    }
}

impl Drop for LayoutNode {
    fn drop(&mut self) {
        // TODO: drop measure

        unsafe { YGNodeFree(self.0) }
    }
}

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
