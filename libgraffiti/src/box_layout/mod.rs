use crate::commons::{SurfaceId, Bounds, Border};
use crate::text_layout::{Text};

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

    fn calculate(&mut self, measure_text: &mut dyn FnMut(SurfaceId, f32) -> (f32, f32));

    fn resize(&mut self, width: i32, height: i32);

    // TODO: not sure if it's necessary for the picker but for rendering
    // we could be fine with <T: Index<SurfaceId>> because the bounds
    // are looked up only once for each surface context so technically,
    // it doesn't have to be continuous slice in memory
    fn get_bounds(&self) -> &[Bounds];
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum Dimension {
    Undefined,
    Auto,
    Points { value: f32 },
    Percent { value: f32 },
}

#[derive(Debug, Clone, Copy)]
pub enum AlignProp {
    AlignContent,
    AlignItems,
    AlignSelf,
    JustifyContent,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum FlexDirection {
    Column,
    ColumnReverse,
    Row,
    RowReverse,
}

#[derive(Debug, Clone, Copy)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[cfg(feature = "stretch")]
pub mod stretch;

pub mod yoga;

#[cfg(feature = "stretch")]
pub type BoxLayoutImpl = stretch::StretchLayout;

#[cfg(not(feature = "stretch"))]
pub type BoxLayoutImpl = yoga::YogaLayout;