use crate::commons::{ElementId, TextId, ElementChild, Bounds};

/// Generic trait for box layout trees/solvers:
/// - keep & organize layout nodes
/// - set layout style props
/// - calculate & provide box bounds for rendering
/// - bounds are relative to their parents
pub trait BoxLayoutTree {
    fn realloc(&mut self, elements_count: ElementId, texts_count: TextId);

    // hiearchy
    fn insert_at(&mut self, parent: ElementId, child: ElementChild, index: usize);
    fn remove_child(&mut self, parent: ElementId, child: ElementChild);

    // prop setters (supported layout features)
    // we could publish Node trait too but this way impls
    // have flexibility to change whatever state they need to

    fn set_display(&mut self, element: ElementId, v: Display);

    fn set_width(&mut self, element: ElementId, v: Dimension);
    fn set_height(&mut self, element: ElementId, v: Dimension);
    fn set_min_width(&mut self, element: ElementId, v: Dimension);
    fn set_min_height(&mut self, element: ElementId, v: Dimension);
    fn set_max_width(&mut self, element: ElementId, v: Dimension);
    fn set_max_height(&mut self, element: ElementId, v: Dimension);

    fn set_top(&mut self, element: ElementId, v: Dimension);
    fn set_right(&mut self, element: ElementId, v: Dimension);
    fn set_bottom(&mut self, element: ElementId, v: Dimension);
    fn set_left(&mut self, element: ElementId, v: Dimension);

    fn set_margin_top(&mut self, element: ElementId, v: Dimension);
    fn set_margin_right(&mut self, element: ElementId, v: Dimension);
    fn set_margin_bottom(&mut self, element: ElementId, v: Dimension);
    fn set_margin_left(&mut self, element: ElementId, v: Dimension);

    fn set_padding_top(&mut self, element: ElementId, v: Dimension);
    fn set_padding_right(&mut self, element: ElementId, v: Dimension);
    fn set_padding_bottom(&mut self, element: ElementId, v: Dimension);
    fn set_padding_left(&mut self, element: ElementId, v: Dimension);

    fn set_border_top(&mut self, element: ElementId, v: f32);
    fn set_border_right(&mut self, element: ElementId, v: f32);
    fn set_border_bottom(&mut self, element: ElementId, v: f32);
    fn set_border_left(&mut self, element: ElementId, v: f32);

    fn set_flex_grow(&mut self, element: ElementId, v: f32);
    fn set_flex_shrink(&mut self, element: ElementId, v: f32);
    fn set_flex_basis(&mut self, element: ElementId, v: Dimension);
    fn set_flex_direction(&mut self, element: ElementId, v: FlexDirection);
    fn set_flex_wrap(&mut self, element: ElementId, v: FlexWrap);

    fn set_align_self(&mut self, element: ElementId, v: Align);
    fn set_align_content(&mut self, element: ElementId, v: Align);
    fn set_align_items(&mut self, element: ElementId, v: Align);
    fn set_justify_content(&mut self, element: ElementId, v: Align);

    fn mark_text_dirty(&mut self, text: TextId);

    fn calculate(&mut self, element: ElementId, size: (f32, f32), measure_text_fn: &mut dyn FnMut(TextId, f32) -> (f32, f32));

    fn get_element_bounds(&self, element: ElementId) -> Bounds;
    fn get_text_bounds(&self, element: ElementId) -> Bounds;
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
