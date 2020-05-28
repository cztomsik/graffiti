// x easy to test
// x return (impl-specific) handles
// x keep & organize layout nodes
// x set all props at once
// x calculate & provide box bounds for rendering
// x bounds relative to their parents
// x node/leaf type cannot be changed (but you can always create a new one and replace it)

#![allow(unused)]

pub trait LayoutEngine {
    type LayoutNodeId;

    fn create_node(&mut self, style: &LayoutStyle) -> Self::LayoutNodeId;
    fn set_style(&mut self, node: Self::LayoutNodeId, style: &LayoutStyle);
    fn insert_child(&mut self, parent: Self::LayoutNodeId, index: usize, child: Self::LayoutNodeId);
    fn remove_child(&mut self, parent: Self::LayoutNodeId, child: Self::LayoutNodeId);

    fn create_leaf(&mut self, measure_fn: impl Fn(f32) -> (f32, f32)) -> Self::LayoutNodeId;
    fn mark_dirty(&mut self, leaf: Self::LayoutNodeId);

    fn calculate(&mut self, node: Self::LayoutNodeId, size: (f32, f32));
    fn get_offset(&self, node: Self::LayoutNodeId) -> (f32, f32);
    fn get_size(&self, node: Self::LayoutNodeId) -> (f32, f32);

    fn free_node(&mut self, node: Self::LayoutNodeId);
}

// supported features
pub struct LayoutStyle {
    display: Display,
    overflow: Overflow,

    flex: Flex,
    width: Dimension,
    height: Dimension,
    min_width: Dimension,
    min_height: Dimension,
    max_width: Dimension,
    max_height: Dimension,

    padding: Rect<Dimension>,
    margin: Rect<Dimension>,
    // f32 for now (bc. yoga/renderer)
    border: Rect<f32>,

    flex_flow: FlexFlow,
    //place_items: PlaceItems,
    //place_content: PlaceContent,
    //place_self: PlaceSelf,
    align_items: Align,
    align_content: Align,
    justify_content: Justify,
    align_self: Align,

    //position: Position,
    top: Dimension,
    right: Dimension,
    bottom: Dimension,
    left: Dimension,
}

impl LayoutStyle {
    // it doesn't have to be here but it's useful for testing
    // rust can't derive `Default` for enums and 3rd party crates are not worth it
    pub const DEFAULT: Self = Self {
        // TODO: Inline
        display: Display::Block,
        overflow: Overflow::Visible,

        flex: Flex {
            grow: 0.,
            shrink: 0.,
            basis: Dimension::Auto,
        },
        width: Dimension::Auto,
        height: Dimension::Auto,
        min_width: Dimension::Undefined,
        min_height: Dimension::Undefined,
        max_width: Dimension::Undefined,
        max_height: Dimension::Undefined,

        padding: Rect::<Dimension>::ZERO,
        margin: Rect::<Dimension>::ZERO,
        // should be "medium" but only if style != solid
        border: Rect::<f32>::ZERO,

        flex_flow: FlexFlow {
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
        },
        align_items: Align::Stretch,
        align_content: Align::Stretch,
        justify_content: Justify::FlexStart,
        align_self: Align::Auto,

        // position: Position::Static,
        top: Dimension::Undefined,
        right: Dimension::Undefined,
        bottom: Dimension::Undefined,
        left: Dimension::Undefined,
    };
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
pub struct Rect<T> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

impl Rect<f32> {
    pub const ZERO: Self = Self {
        top: 0.,
        right: 0.,
        bottom: 0.,
        left: 0.,
    };
}

impl Rect<Dimension> {
    pub const ZERO: Self = Self {
        top: Dimension::Px(0.),
        right: Dimension::Px(0.),
        bottom: Dimension::Px(0.),
        left: Dimension::Px(0.),
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Flex {
    pub grow: f32,
    pub shrink: f32,
    pub basis: Dimension,
}

#[derive(Debug, Clone, Copy)]
pub struct FlexFlow {
    pub direction: FlexDirection,
    pub wrap: FlexWrap,
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
}

#[derive(Debug, Clone, Copy)]
pub enum Justify {
    FlexStart,
    Center,
    FlexEnd,
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
pub type LayoutEngineImpl = yoga::YogaLayoutEngine;

// generic tests, shared for all impls
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node() {
        let mut le = LayoutEngineImpl::new();

        let parent = le.create_node(&LayoutStyle::DEFAULT);
        let child = le.create_node(&LayoutStyle::DEFAULT);

        le.insert_child(parent, 0, child);
        le.calculate(parent, (100., 100.));

        le.remove_child(parent, child);
        le.calculate(parent, (100., 100.));
    }

    #[test]
    fn leaf() {
        let mut le = LayoutEngineImpl::new();

        let leaf = le.create_leaf(|_| (10., 10.));

        le.mark_dirty(leaf);
        le.calculate(leaf, (100., 100.));

        // takes whole row (in this case)
        assert_eq!(le.get_size(leaf), (100., 10.));
    }

    #[test]
    fn calculate() {
        let mut le = LayoutEngineImpl::new();

        let root = le.create_node(&LayoutStyle {
            left: Dimension::Px(10.),
            top: Dimension::Px(10.),
            width: Dimension::Percent(50.),
            height: Dimension::Px(10.),
            ..LayoutStyle::DEFAULT
        });

        le.calculate(root, (100., 100.));

        assert_eq!(le.get_offset(root), (10., 10.));
        assert_eq!(le.get_size(root), (50., 10.));
    }
}
