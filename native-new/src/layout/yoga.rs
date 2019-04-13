#[cfg(test)]
use ordered_float::OrderedFloat;
use std::f32;
use yoga::{
    Align, Context, Direction, FlexDirection as YogaFlexDirection, FlexStyle, MeasureMode,
    Node as YogaNode, NodeRef, StyleUnit, Wrap,
};

use super::LayoutTree;
use crate::api::{
    Rect, Dimension, Dimensions, Flex, FlexAlign, FlexDirection, FlexWrap, Flow, JustifyContent,
    Size, Text,
};
use crate::text::{PangoService, TextLayoutAlgo, LaidText};
use crate::Id;
use yoga::types::Justify;
use std::collections::BTreeMap;

pub struct YogaTree {
    yoga_nodes: Vec<YogaNode>,
    text_layout_algo: PangoService,
    text_layouts: BTreeMap<Id, LaidText>
}

impl YogaTree {
    pub fn new() -> Self {
        YogaTree {
            yoga_nodes: vec![],
            text_layout_algo: PangoService::new(),
            text_layouts: BTreeMap::new()
        }
    }
}

impl LayoutTree for YogaTree {
    fn alloc(&mut self) {
        self.yoga_nodes.push(YogaNode::new())
    }

    fn append_child(&mut self, parent: Id, child: Id) {
        let (parent, child) = get_two_muts(&mut self.yoga_nodes, parent, child);

        let index = parent.get_child_count();
        parent.insert_child(child, index);
    }

    fn remove_child(&mut self, parent: Id, child: Id) {
        let (parent, child) = get_two_muts(&mut self.yoga_nodes, parent, child);

        parent.remove_child(child);
    }

    // easier with index rather than with Id
    fn insert_at(&mut self, parent: Id, child: Id, index: u32) {
        let (parent, child) = get_two_muts(&mut self.yoga_nodes, parent, child);

        parent.insert_child(child, index);
    }

    fn set_size(&mut self, id: Id, size: Size) {
        self.yoga_nodes[id].apply_styles(&[
            FlexStyle::Width(size.0.into()),
            FlexStyle::Height(size.1.into()),
        ])
    }

    fn set_flex(&mut self, id: Id, flex: Flex) {
        self.yoga_nodes[id].apply_styles(&[
            FlexStyle::FlexGrow(flex.flex_grow.into()),
            FlexStyle::FlexShrink(flex.flex_shrink.into()),
            FlexStyle::FlexBasis(flex.flex_basis.into()),
        ]);
    }

    fn set_flow(&mut self, id: Id, flow: Flow) {
        self.yoga_nodes[id].apply_styles(&[
            FlexStyle::FlexDirection(flow.flex_direction.into()),
            FlexStyle::FlexWrap(flow.flex_wrap.into()),
            FlexStyle::JustifyContent(flow.justify_content.into()),
            FlexStyle::AlignContent(flow.align_content.into()),
            FlexStyle::AlignItems(flow.align_items.into()),
        ]);
    }

    fn set_padding(&mut self, id: Id, padding: Dimensions) {
        self.yoga_nodes[id].apply_styles(&[
            FlexStyle::PaddingTop(padding.0.into()),
            FlexStyle::PaddingRight(padding.1.into()),
            FlexStyle::PaddingBottom(padding.2.into()),
            FlexStyle::PaddingLeft(padding.3.into()),
        ]);
    }

    fn set_margin(&mut self, id: Id, margin: Dimensions) {
        self.yoga_nodes[id].apply_styles(&[
            FlexStyle::MarginTop(margin.0.into()),
            FlexStyle::MarginRight(margin.1.into()),
            FlexStyle::MarginBottom(margin.2.into()),
            FlexStyle::MarginLeft(margin.3.into()),
        ]);
    }

    fn set_text<'svc>(&mut self, id: Id, text: Option<Text>) {
        // yoganode context has static lifetime and we need to access pango and text_layouts somehow
        // should be safe but I might be wrong OFC
        let tree_ref: &'static mut YogaTree = get_static_ref(self);

        let node = &mut self.yoga_nodes[id];

        if let Some(text) = text {
            node.set_measure_func(Some(measure_text_node));
            node.mark_dirty();
            node.set_context(Some(Context::new(MeasureContext(tree_ref, id, text))));
        } else {
            node.set_measure_func(None);
            node.set_context(None);
            self.text_layouts.remove(&id);
        }
    }

    fn calculate(&mut self) {
        self.yoga_nodes[0].calculate_layout(f32::MAX, f32::MAX, Direction::LTR);
    }

    fn computed_layout(&self, id: Id) -> Rect {
        let n = &self.yoga_nodes[id];

        Rect(
            n.get_layout_left(),
            n.get_layout_top(),
            n.get_layout_width(),
            n.get_layout_height()
        )
    }

    fn text_layout(&self, id: Id) -> LaidText {
        self.text_layouts.get(&id).expect("no text on the surface").clone()
    }
}

extern "C" fn measure_text_node(
    node_ref: NodeRef,
    w: f32,
    wm: MeasureMode,
    _h: f32,
    _hm: MeasureMode,
) -> yoga::Size {
    let ctx = YogaNode::get_context_mut(&node_ref).expect("no context found");
    let MeasureContext(tree, id, text) = ctx
        .downcast_mut::<MeasureContext>()
        .expect("not a measure context");

    let max_width = match wm {
        MeasureMode::Exactly => Some(w),
        MeasureMode::AtMost => Some(w),
        MeasureMode::Undefined => None,
    };

    let layout = tree.text_layout_algo.layout_text(&text, max_width);

    let width = match wm {
        MeasureMode::Exactly => w,
        MeasureMode::AtMost => layout.width,
        MeasureMode::Undefined => layout.width,
    };

    let size = yoga::Size { width, height: (layout.lines as f32) * text.line_height };

    // save the result so it can be queried later
    tree.text_layouts.insert(*id, layout);

    size
}

struct MeasureContext (
    pub &'static mut YogaTree,
    pub Id,
    pub Text
);

impl Into<StyleUnit> for Dimension {
    fn into(self) -> StyleUnit {
        match self {
            Dimension::Auto => StyleUnit::Auto,
            Dimension::Percent(f) => StyleUnit::Percent(f.into()),
            Dimension::Point(f) => StyleUnit::Point(f.into()),
        }
    }
}

impl Into<YogaFlexDirection> for FlexDirection {
    fn into(self) -> YogaFlexDirection {
        match self {
            FlexDirection::Column => YogaFlexDirection::Column,
            FlexDirection::ColumnReverse => YogaFlexDirection::ColumnReverse,
            FlexDirection::Row => YogaFlexDirection::Row,
            FlexDirection::RowReverse => YogaFlexDirection::RowReverse,
        }
    }
}

impl Into<Align> for FlexAlign {
    fn into(self) -> Align {
        match self {
            FlexAlign::Auto => Align::Auto,
            FlexAlign::Baseline => Align::Baseline,
            FlexAlign::Center => Align::Center,
            FlexAlign::FlexStart => Align::FlexStart,
            FlexAlign::FlexEnd => Align::FlexEnd,
            FlexAlign::SpaceAround => Align::SpaceAround,
            FlexAlign::SpaceBetween => Align::SpaceBetween,
            FlexAlign::Stretch => Align::Stretch,
        }
    }
}

impl Into<Justify> for JustifyContent {
    fn into(self) -> Justify {
        match self {
            JustifyContent::Center => Justify::Center,
            JustifyContent::FlexStart => Justify::FlexStart,
            JustifyContent::FlexEnd => Justify::FlexEnd,
            JustifyContent::SpaceAround => Justify::SpaceAround,
            JustifyContent::SpaceBetween => Justify::SpaceBetween,
            JustifyContent::SpaceEvenly => Justify::SpaceEvenly,
        }
    }
}

impl Into<Wrap> for FlexWrap {
    fn into(self) -> Wrap {
        match self {
            FlexWrap::Wrap => Wrap::Wrap,
            FlexWrap::WrapReverse => Wrap::WrapReverse,
            FlexWrap::NoWrap => Wrap::NoWrap,
        }
    }
}

// mutably borrow two items at once
pub fn get_two_muts<T>(vec: &mut Vec<T>, first: usize, second: usize) -> (&mut T, &mut T) {
    let len = vec.len();

    assert!(first < len);
    assert!(second < len);
    assert_ne!(first, second);

    let ptr = vec.as_mut_ptr();

    unsafe { (&mut *ptr.add(first), &mut *ptr.add(second)) }
}

pub fn get_static_ref(tree: &mut YogaTree) -> &'static mut YogaTree {
    unsafe { std::mem::transmute(tree) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_svc(count: usize) -> YogaLayoutService {
        let mut svc = YogaLayoutService::new();

        for _n in 0..count {
            svc.alloc();
        }

        svc
    }

    #[test]
    fn test_append_child() {
        let mut svc = test_svc(2);
        let parent = 0;
        let child = 1;

        assert_eq!(svc.yoga_nodes.get(parent).get_child_count(), 0);
        svc.append_child(parent, child);
        assert_eq!(svc.yoga_nodes.get(parent).get_child_count(), 1);
    }

    #[test]
    fn test_layout_set_size() {
        let mut svc = test_svc(1);
        let id = 0;

        svc.set_size(id, Size(Dimension::Point(100.), Dimension::Percent(100.)));

        assert_eq!(
            svc.yoga_nodes.get(id).get_style_width(),
            StyleUnit::Point(OrderedFloat::from(100.))
        );
        assert_eq!(
            svc.yoga_nodes.get(id).get_style_height(),
            StyleUnit::Percent(OrderedFloat::from(100.))
        );
    }
}
