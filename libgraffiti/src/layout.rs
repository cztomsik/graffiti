

#[derive(Debug, Clone, Copy)]
pub enum Display { None, Inline, Block, Flex }

#[derive(Debug, Clone, Copy)]
pub enum Dimension { Auto, Px(f32), /*Fraction*/ Percent(f32) }

#[derive(Debug, Clone, Copy)]
pub struct Size<T: Copy> { pub width: T, pub height: T }

#[derive(Debug, Clone, Copy)]
pub struct Rect<T: Copy> { pub top: T, pub right: T, pub bottom: T, pub left: T }

#[derive(Debug, Clone, Copy)]
pub struct LayoutStyle {
    pub display: Display,
    pub size: Size<Dimension>,
    pub min_size: Size<Dimension>,
    pub max_size: Size<Dimension>,
    pub padding: Rect<Dimension>,
    pub margin: Rect<Dimension>,
    pub border: Rect<Dimension>,
}

// TODO: vw, vh, vmin, vmax, rem
struct Ctx {}

impl Ctx {
    fn resolve(&self, dim: Dimension, base: f32) -> f32 {
        match dim {
            Dimension::Px(v) => v,
            Dimension::Percent(v) => base * v,
            _ => f32::NAN
        }
    }

    fn resolve_size(&self, size: Size<Dimension>, parent_size: Size<f32>) -> Size<f32> {
        Size { width: self.resolve(size.width, parent_size.width), height: self.resolve(size.height, parent_size.height) }
    }

    fn resolve_rect(&self, rect: Rect<Dimension>, base: f32) -> Rect<f32> {
        Rect { top: self.resolve(rect.top, base), right: self.resolve(rect.top, base), bottom: self.resolve(rect.top, base), left: self.resolve(rect.top, base) }
    }

}



/*
// TODO: eventually replace this with own layout engine
//
// x independent, easy to test
// x set all props at once
// x calculate & provide box bounds for rendering
// x bounds relative to their parents
// x node/leaf type cannot be changed

#![allow(unused)]

use graffiti_yoga::*;
use std::convert::TryInto;

pub struct LayoutStyle {
    // size
    pub width: Dimension,
    pub height: Dimension,
    pub min_width: Dimension,
    pub min_height: Dimension,
    pub max_width: Dimension,
    pub max_height: Dimension,

    // padding
    pub padding_top: Dimension,
    pub padding_right: Dimension,
    pub padding_bottom: Dimension,
    pub padding_left: Dimension,

    // margin
    pub margin_top: Dimension,
    pub margin_right: Dimension,
    pub margin_bottom: Dimension,
    pub margin_left: Dimension,

    // border
    pub border_top: f32,
    pub border_right: f32,
    pub border_bottom: f32,
    pub border_left: f32,

    // flex
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Dimension,
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub align_content: Align,
    pub align_items: Align,
    pub align_self: Align,
    pub justify_content: Justify,

    // position
    pub position: Position,
    pub top: Dimension,
    pub right: Dimension,
    pub bottom: Dimension,
    pub left: Dimension,

    // overflow
    pub overflow_x: Overflow,
    pub overflow_y: Overflow,

    // other
    pub display: Display,
}

impl Default for LayoutStyle {
    fn default() -> Self {
        Self {
            // size
            width: Dimension::Auto,
            height: Dimension::Auto,
            min_width: Dimension::Undefined,
            min_height: Dimension::Undefined,
            max_width: Dimension::Undefined,
            max_height: Dimension::Undefined,

            // padding
            padding_top: Dimension::Undefined,
            padding_right: Dimension::Undefined,
            padding_bottom: Dimension::Undefined,
            padding_left: Dimension::Undefined,

            // margin
            margin_top: Dimension::Undefined,
            margin_right: Dimension::Undefined,
            margin_bottom: Dimension::Undefined,
            margin_left: Dimension::Undefined,

            // border
            border_top: 0.,
            border_right: 0.,
            border_bottom: 0.,
            border_left: 0.,

            // flex
            flex_grow: 0.,
            flex_shrink: 1.,
            flex_basis: Dimension::Auto,
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::NoWrap,
            align_content: Align::Stretch,
            align_items: Align::Stretch,
            align_self: Align::Auto,
            justify_content: Justify::FlexStart,

            // position
            position: Position::Relative,
            top: Dimension::Undefined,
            right: Dimension::Undefined,
            bottom: Dimension::Undefined,
            left: Dimension::Undefined,

            // overflow
            overflow_x: Overflow::Visible,
            overflow_y: Overflow::Visible,

            // other
            // TODO: default should be "inline"
            display: Display::Flex,
        }
    }
}

pub type Display = YGDisplay;
pub type FlexDirection = YGFlexDirection;
pub type FlexWrap = YGWrap;
pub type Overflow = YGOverflow;
pub type Align = YGAlign;
pub type Justify = YGJustify;
pub type Position = YGPositionType;

pub enum Dimension {
    Undefined,
    Px(f32),
    Percent(f32),
    Auto,
}

#[derive(Debug)]
pub struct LayoutNode(YGNodeRef);

impl LayoutNode {
    pub fn new() -> Self {
        Self(unsafe { YGNodeNew() })
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

    pub fn set_style(&self, style: LayoutStyle) {
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

        // size
        set_dim!(self, style.width; YGNodeStyleSetWidth YGNodeStyleSetWidthPercent YGNodeStyleSetWidthAuto);
        set_dim!(self, style.height; YGNodeStyleSetHeight YGNodeStyleSetHeightPercent YGNodeStyleSetHeightAuto);
        set_dim!(self, style.min_width; YGNodeStyleSetMinWidth YGNodeStyleSetMinWidthPercent);
        set_dim!(self, style.min_height; YGNodeStyleSetMinHeight YGNodeStyleSetMinHeightPercent);
        set_dim!(self, style.max_width; YGNodeStyleSetMaxWidth YGNodeStyleSetMaxWidthPercent);
        set_dim!(self, style.max_height; YGNodeStyleSetMaxHeight YGNodeStyleSetMaxHeightPercent);

        // padding
        set_edge_dim!(self, YGEdge::Top, style.padding_top; YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent);
        set_edge_dim!(self, YGEdge::Right, style.padding_right; YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent);
        set_edge_dim!(self, YGEdge::Bottom, style.padding_bottom; YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent);
        set_edge_dim!(self, YGEdge::Left, style.padding_left; YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent);

        // margin
        set_edge_dim!(self, YGEdge::Top, style.margin_top; YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto);
        set_edge_dim!(self, YGEdge::Right, style.margin_right; YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto);
        set_edge_dim!(self, YGEdge::Bottom, style.margin_bottom; YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto);
        set_edge_dim!(self, YGEdge::Left, style.margin_left; YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto);

        // border
        unsafe { YGNodeStyleSetBorder(self.0, YGEdge::Top, style.border_top) }
        unsafe { YGNodeStyleSetBorder(self.0, YGEdge::Right, style.border_right) }
        unsafe { YGNodeStyleSetBorder(self.0, YGEdge::Bottom, style.border_bottom) }
        unsafe { YGNodeStyleSetBorder(self.0, YGEdge::Left, style.border_left) }

        // position
        unsafe { YGNodeStyleSetPositionType(self.0, style.position) }
        set_edge_dim!(self, YGEdge::Top, style.top; YGNodeStyleSetPosition YGNodeStyleSetPositionPercent);
        set_edge_dim!(self, YGEdge::Right, style.right; YGNodeStyleSetPosition YGNodeStyleSetPositionPercent);
        set_edge_dim!(self, YGEdge::Bottom, style.bottom; YGNodeStyleSetPosition YGNodeStyleSetPositionPercent);
        set_edge_dim!(self, YGEdge::Left, style.left; YGNodeStyleSetPosition YGNodeStyleSetPositionPercent);

        // flex
        unsafe { YGNodeStyleSetFlexGrow(self.0, style.flex_grow) }
        unsafe { YGNodeStyleSetFlexShrink(self.0, style.flex_shrink) }
        set_dim!(self, style.flex_basis; YGNodeStyleSetFlexBasis YGNodeStyleSetFlexBasisPercent YGNodeStyleSetFlexBasisAuto);
        unsafe { YGNodeStyleSetFlexDirection(self.0, style.flex_direction) }
        unsafe { YGNodeStyleSetFlexWrap(self.0, style.flex_wrap) }
        unsafe { YGNodeStyleSetAlignContent(self.0, style.align_content) }
        unsafe { YGNodeStyleSetAlignItems(self.0, style.align_items) }
        unsafe { YGNodeStyleSetAlignSelf(self.0, style.align_self) }
        unsafe { YGNodeStyleSetJustifyContent(self.0, style.justify_content) }

        // TODO: overflow

        // other
        unsafe { YGNodeStyleSetDisplay(self.0, style.display) }
    }

    pub fn insert_child(&self, child: &LayoutNode, index: usize) {
        unsafe { YGNodeInsertChild(self.0, child.0, index.try_into().unwrap()) }
    }

    pub fn remove_child(&self, child: &LayoutNode) {
        unsafe { YGNodeRemoveChild(self.0, child.0) }
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
*/

