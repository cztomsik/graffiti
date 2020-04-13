// x easy to test
//   x pass closure for measuring
// x return (impl-specific) handles
// x keep & organize layout nodes
// x set props
// x calculate & provide box bounds for rendering
// x bounds relative to their parents

use crate::commons::Bounds;

pub trait BoxLayoutTree {
    type LayoutNodeId;
    type MeasureKey;

    fn create_node(&mut self, measure_key: Option<Self::MeasureKey>) -> Self::LayoutNodeId;

    // children

    fn insert_child(&mut self, parent: Self::LayoutNodeId, index: usize, child: Self::LayoutNodeId);
    fn remove_child(&mut self, parent: Self::LayoutNodeId, child: Self::LayoutNodeId);

    // prop setters (supported layout features)

    fn set_display(&mut self, node: Self::LayoutNodeId, v: Display);
    fn set_overflow(&mut self, node: Self::LayoutNodeId, v: Overflow);

    fn set_width(&mut self, node: Self::LayoutNodeId, v: Dimension);
    fn set_height(&mut self, node: Self::LayoutNodeId, v: Dimension);
    fn set_min_width(&mut self, node: Self::LayoutNodeId, v: Dimension);
    fn set_min_height(&mut self, node: Self::LayoutNodeId, v: Dimension);
    fn set_max_width(&mut self, node: Self::LayoutNodeId, v: Dimension);
    fn set_max_height(&mut self, node: Self::LayoutNodeId, v: Dimension);

    fn set_top(&mut self, node: Self::LayoutNodeId, v: Dimension);
    fn set_right(&mut self, node: Self::LayoutNodeId, v: Dimension);
    fn set_bottom(&mut self, node: Self::LayoutNodeId, v: Dimension);
    fn set_left(&mut self, node: Self::LayoutNodeId, v: Dimension);

    fn set_margin_top(&mut self, node: Self::LayoutNodeId, v: Dimension);
    fn set_margin_right(&mut self, node: Self::LayoutNodeId, v: Dimension);
    fn set_margin_bottom(&mut self, node: Self::LayoutNodeId, v: Dimension);
    fn set_margin_left(&mut self, node: Self::LayoutNodeId, v: Dimension);

    fn set_padding_top(&mut self, node: Self::LayoutNodeId, v: Dimension);
    fn set_padding_right(&mut self, node: Self::LayoutNodeId, v: Dimension);
    fn set_padding_bottom(&mut self, node: Self::LayoutNodeId, v: Dimension);
    fn set_padding_left(&mut self, node: Self::LayoutNodeId, v: Dimension);

    fn set_border_top(&mut self, node: Self::LayoutNodeId, v: f32);
    fn set_border_right(&mut self, node: Self::LayoutNodeId, v: f32);
    fn set_border_bottom(&mut self, node: Self::LayoutNodeId, v: f32);
    fn set_border_left(&mut self, node: Self::LayoutNodeId, v: f32);

    fn set_flex_grow(&mut self, node: Self::LayoutNodeId, v: f32);
    fn set_flex_shrink(&mut self, node: Self::LayoutNodeId, v: f32);
    fn set_flex_basis(&mut self, node: Self::LayoutNodeId, v: Dimension);
    fn set_flex_direction(&mut self, node: Self::LayoutNodeId, v: FlexDirection);
    fn set_flex_wrap(&mut self, node: Self::LayoutNodeId, v: FlexWrap);

    fn set_align_self(&mut self, node: Self::LayoutNodeId, v: Align);
    fn set_align_content(&mut self, node: Self::LayoutNodeId, v: Align);
    fn set_align_items(&mut self, node: Self::LayoutNodeId, v: Align);
    fn set_justify_content(&mut self, node: Self::LayoutNodeId, v: Align);

    fn mark_dirty(&mut self, node: Self::LayoutNodeId);

    fn calculate(&mut self, node: Self::LayoutNodeId, size: (f32, f32), measure_fn: &mut dyn FnMut(Self::MeasureKey, f32) -> (f32, f32));

    fn get_bounds(&self, node: Self::LayoutNodeId) -> Bounds;
}

#[derive(Debug, Clone, Copy)]
pub enum Display {
    None,
    Block,
    Flex,
}

#[derive(Debug, Clone, Copy)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
}

#[derive(Debug, Clone, Copy)]
pub enum Dimension {
    Undefined,
    Auto,
    Px(f32),
    Percent(f32),
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
pub type BoxLayoutImpl = yoga::YogaLayoutTree;
