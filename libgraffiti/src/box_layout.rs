use crate::commons::{SurfaceId, Bounds};

/// Generic (to avoid `dyn`) trait for box layout solvers.
///
/// - keep layout nodes for each surface
/// - set layout style props
/// - calculate & provide box bounds for rendering
///
/// Only flexbox is expected for now, others might be added in the future
///
/// The text layout is a separate thing and the only relation is that
/// the box layout (sometimes) needs to measure the text content to determine box sizes.
/// For this purpose the `measure_text` callback is provided to the `calculate` method.
pub trait BoxLayout<N: BoxLayoutNode> {
    fn alloc(&mut self);
    fn insert_at(&mut self, parent: SurfaceId, child: SurfaceId, index: usize);
    fn remove_child(&mut self, parent: SurfaceId, child: SurfaceId);

    fn get_node_mut(&mut self, id: SurfaceId) -> &mut N;

    // TODO: this is temporary
    fn set_measure_text(&mut self, id: SurfaceId, measure: bool);

    fn calculate(&mut self);

    fn resize(&mut self, width: i32, height: i32);

    // TODO: not sure if it's necessary for the picker but for rendering
    // we could be fine with <T: Index<SurfaceId>> because the bounds
    // are looked up only once for each surface context so technically,
    // it doesn't have to be continuous slice in memory
    fn get_bounds(&self) -> &[Bounds];
}

pub trait BoxLayoutNode {
    fn set_display(&mut self, v: Display);

    fn set_width(&mut self, v: Dimension);
    fn set_height(&mut self, v: Dimension);
    fn set_min_width(&mut self, v: Dimension);
    fn set_min_height(&mut self, v: Dimension);
    fn set_max_width(&mut self, v: Dimension);
    fn set_max_height(&mut self, v: Dimension);

    fn set_top(&mut self, v: Dimension);
    fn set_right(&mut self, v: Dimension);
    fn set_bottom(&mut self, v: Dimension);
    fn set_left(&mut self, v: Dimension);

    fn set_margin_top(&mut self, v: Dimension);
    fn set_margin_right(&mut self, v: Dimension);
    fn set_margin_bottom(&mut self, v: Dimension);
    fn set_margin_left(&mut self, v: Dimension);

    fn set_padding_top(&mut self, v: Dimension);
    fn set_padding_right(&mut self, v: Dimension);
    fn set_padding_bottom(&mut self, v: Dimension);
    fn set_padding_left(&mut self, v: Dimension);

    fn set_border_top(&mut self, v: f32);
    fn set_border_right(&mut self, v: f32);
    fn set_border_bottom(&mut self, v: f32);
    fn set_border_left(&mut self, v: f32);

    fn set_flex_grow(&mut self, v: f32);
    fn set_flex_shrink(&mut self, v: f32);
    fn set_flex_basis(&mut self, v: Dimension);
    fn set_flex_direction(&mut self, v: FlexDirection);
    fn set_flex_wrap(&mut self, v: FlexWrap);

    fn set_align_self(&mut self, v: Align);
    fn set_align_content(&mut self, v: Align);
    fn set_align_items(&mut self, v: Align);
    fn set_justify_content(&mut self, v: Align);
}

#[derive(Debug, Clone, Copy)]
pub enum Display {
    None,
    Flex,
    Block,
}

#[derive(Debug, Clone, Copy)]
pub enum Dimension {
    Undefined,
    Auto,
    Px { value: f32 },
    Percent { value: f32 },
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

pub mod yoga;
#[cfg(not(feature = "stretch"))]
pub type BoxLayoutImpl = yoga::YogaLayout;

#[cfg(feature = "stretch")]
pub mod stretch;
#[cfg(feature = "stretch")]
pub type BoxLayoutImpl = stretch::StretchLayout;
