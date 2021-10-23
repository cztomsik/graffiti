

#[derive(Debug, Clone, Copy)]
pub enum Display { None, Inline, Block, Flex }

#[derive(Debug, Clone, Copy)]
pub enum Dimension { Auto, Px(f32), /*Fraction*/ Percent(f32) }

#[derive(Debug, Clone, Copy)]
pub struct Size<T: Copy> { pub width: T, pub height: T }

impl Size<Dimension> {
    pub const AUTO: Self = Self { width: Dimension::Auto, height: Dimension::Auto };
}

#[derive(Debug, Clone, Copy)]
pub struct Rect<T: Copy> { pub top: T, pub right: T, pub bottom: T, pub left: T }

impl Rect<Dimension> {
    pub const ZERO: Self = Self { top: Dimension::Px(0.), right: Dimension::Px(0.), bottom: Dimension::Px(0.), left: Dimension::Px(0.) };
}

#[derive(Debug, Clone, Copy)]
pub enum Align { Auto, FlexStart, Center, FlexEnd, Stretch, Baseline, SpaceBetween, SpaceAround }

#[derive(Debug, Clone, Copy)]
pub enum Justify { FlexStart, Center, FlexEnd, SpaceBetween, SpaceAround, SpaceEvenly }

#[derive(Debug, Clone, Copy)]
pub enum FlexDirection { Row, Column }

#[derive(Debug, Clone, Copy)]
pub enum FlexWrap { NoWrap, Wrap }

#[derive(Debug, Clone, Copy)]
pub struct LayoutStyle {
    pub display: Display,
    pub size: Size<Dimension>,
    pub min_size: Size<Dimension>,
    pub max_size: Size<Dimension>,
    pub padding: Rect<Dimension>,
    pub margin: Rect<Dimension>,
    pub border: Rect<Dimension>,

    // flex & grid (not supported ATM)
    pub align_self: Align,
    pub align_content: Align,
    pub align_items: Align,
    pub justify_content: Justify,

    // flex
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Dimension,
}

impl Default for LayoutStyle {
    fn default() -> Self {
        Self {
            display: Display::Inline,
            size: Size::AUTO,
            min_size: Size::AUTO,
            max_size: Size::AUTO,
            padding: Rect::ZERO,
            margin: Rect::ZERO,
            border: Rect::ZERO,

            align_self: Align::Auto,
            align_items: Align::Stretch,
            align_content: Align::Stretch,
            justify_content: Justify::FlexStart,

            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::NoWrap,
            flex_grow: 0.,
            flex_shrink: 1.,
            flex_basis: Dimension::Auto,
        }
    }
}

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

    // position
    pub position: Position,
    pub top: Dimension,
    pub right: Dimension,
    pub bottom: Dimension,
    pub left: Dimension,

    // overflow
    pub overflow_x: Overflow,
    pub overflow_y: Overflow,
}

impl Default for LayoutStyle {
    fn default() -> Self {
        Self {
            // position
            position: Position::Relative,
            top: Dimension::Undefined,
            right: Dimension::Undefined,
            bottom: Dimension::Undefined,
            left: Dimension::Undefined,

            // overflow
            overflow_x: Overflow::Visible,
            overflow_y: Overflow::Visible,
        }
    }
}

pub type Overflow = YGOverflow;
pub type Position = YGPositionType;

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
        // TODO: overflow
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

