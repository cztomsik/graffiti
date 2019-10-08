use crate::commons::{SurfaceId, Bounds, Border};
use crate::text_layout::{Text};
use miniserde::{Deserialize, Serialize};

/// Box layout algo
/// Only flexbox is expected for now, grid might be added in the future
///
/// The text layout is a separate thing and the only relation is that
/// the box layout (sometimes) needs to measure the text content to determine box sizes.
/// For this purpose the `measure_text` callback is provided to the `calculate` method.
pub trait BoxLayout {
    fn alloc(&mut self);

    fn insert_at(&mut self, parent: SurfaceId, child: SurfaceId, index: usize);

    fn remove_child(&mut self, parent: SurfaceId, child: SurfaceId);

    fn set_dimension(&mut self, surface: SurfaceId, prop: DimensionProp, value: Dimension);

    fn set_align(&mut self, surface: SurfaceId, prop: AlignProp, value: Align);

    fn set_flex_direction(&mut self, surface: SurfaceId, value: FlexDirection);

    fn set_flex_wrap(&mut self, surface: SurfaceId, value: FlexWrap);

    // separate because rendering doesn't need to test dimensions then
    fn set_border(&mut self, surface: SurfaceId, border: Option<Border>);

    // another separate
    fn set_text(&mut self, surface: SurfaceId, text: Option<Text>);

    fn calculate(&mut self, measure_text: &mut dyn FnMut(SurfaceId, Option<f32>) -> (f32, f32));

    // TODO: not sure if it's necessary for the picker but for rendering
    // we could be fine with <T: Index<SurfaceId>> because the bounds
    // are looked up only once for each surface context so technically,
    // it doesn't have to be continuous slice in memory
    fn get_bounds(&self) -> &[Bounds];
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum DimensionProp {
    Width,
    Height,
    MinWidth,
    MinHeight,
    MaxWidth,
    MaxHeight,

    PaddingLeft,
    PaddingRight,
    PaddingTop,
    PaddingBottom,

    MarginLeft,
    MarginRight,
    MarginTop,
    MarginBottom,

    FlexGrow,
    FlexShrink,
    FlexBasis,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct Dimension {
    // auto when both are None
    point: Option<f32>,
    percent: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum AlignProp {
    AlignContent,
    AlignItems,
    AlignSelf,
    JustifyContent,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum Align {
    Auto,
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
    Baseline,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

pub enum LayoutProp {
    FlexDirection,
    FlexWrap,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum FlexDirection {
    Column,
    ColumnReverse,
    Row,
    RowReverse,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

/*
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Layout {
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub overflow: Overflow,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
}
*/

mod stretch;
pub use self::stretch::StretchLayout;
