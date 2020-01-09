use crate::commons::{Bounds};

/// Generic trait for box layout trees/solvers:
/// - keep & organize layout nodes
/// - set layout style props
/// - calculate & provide box bounds for rendering
///
/// Only flexbox is expected for now
///
/// Sometimes, the width/height needs to be computed (for text).
/// To do that, you need to call `set_measure_key(node, key)` and then you
/// need to provide `measure_fn(key, max_width)` to `calculate()`.
pub trait BoxLayoutTree<NodeId, MeasureKey> {
    fn create_node(&mut self) -> NodeId;
    fn insert_at(&mut self, parent: NodeId, child: NodeId, index: usize);
    fn remove_child(&mut self, parent: NodeId, child: NodeId);

    // prop setters (supported layout features)
    // we could publish Node trait too but this way impls
    // have flexibility to change whatever state they need to

    fn set_display(&mut self, node: NodeId, v: Display);

    fn set_width(&mut self, node: NodeId, v: Dimension);
    fn set_height(&mut self, node: NodeId, v: Dimension);
    fn set_min_width(&mut self, node: NodeId, v: Dimension);
    fn set_min_height(&mut self, node: NodeId, v: Dimension);
    fn set_max_width(&mut self, node: NodeId, v: Dimension);
    fn set_max_height(&mut self, node: NodeId, v: Dimension);

    fn set_top(&mut self, node: NodeId, v: Dimension);
    fn set_right(&mut self, node: NodeId, v: Dimension);
    fn set_bottom(&mut self, node: NodeId, v: Dimension);
    fn set_left(&mut self, node: NodeId, v: Dimension);

    fn set_margin_top(&mut self, node: NodeId, v: Dimension);
    fn set_margin_right(&mut self, node: NodeId, v: Dimension);
    fn set_margin_bottom(&mut self, node: NodeId, v: Dimension);
    fn set_margin_left(&mut self, node: NodeId, v: Dimension);

    fn set_padding_top(&mut self, node: NodeId, v: Dimension);
    fn set_padding_right(&mut self, node: NodeId, v: Dimension);
    fn set_padding_bottom(&mut self, node: NodeId, v: Dimension);
    fn set_padding_left(&mut self, node: NodeId, v: Dimension);

    fn set_border_top(&mut self, node: NodeId, v: f32);
    fn set_border_right(&mut self, node: NodeId, v: f32);
    fn set_border_bottom(&mut self, node: NodeId, v: f32);
    fn set_border_left(&mut self, node: NodeId, v: f32);

    fn set_flex_grow(&mut self, node: NodeId, v: f32);
    fn set_flex_shrink(&mut self, node: NodeId, v: f32);
    fn set_flex_basis(&mut self, node: NodeId, v: Dimension);
    fn set_flex_direction(&mut self, node: NodeId, v: FlexDirection);
    fn set_flex_wrap(&mut self, node: NodeId, v: FlexWrap);

    fn set_align_self(&mut self, node: NodeId, v: Align);
    fn set_align_content(&mut self, node: NodeId, v: Align);
    fn set_align_items(&mut self, node: NodeId, v: Align);
    fn set_justify_content(&mut self, node: NodeId, v: Align);

    fn set_measure_key(&mut self, node: NodeId, key: Option<MeasureKey>);

    // TODO: find out if it's somehow possible to detect what has been changed
    // and provide iterator over those
    // maybe even do parent/child offsetting?
    // not sure if it's possible to do at this level, maybe it should be
    // somewhere else
    fn calculate(&mut self, measure_fn: &mut dyn FnMut(MeasureKey, f32) -> (f32, f32));

    fn resize(&mut self, width: i32, height: i32);

    // TODO: not sure if it's necessary for the picker but for rendering
    // we could be fine with <T: Index<SurfaceId>> because the bounds
    // are looked up only once for each surface context so technically,
    // it doesn't have to be continuous slice in memory
    fn get_bounds(&self) -> &[Bounds];
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
pub type BoxLayoutImpl = yoga::YogaLayoutTree;

#[cfg(feature = "stretch")]
pub mod stretch;
#[cfg(feature = "stretch")]
pub type BoxLayoutImpl = stretch::StretchLayout;
