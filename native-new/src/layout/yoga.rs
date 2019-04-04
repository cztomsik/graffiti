#[cfg(test)]
use ordered_float::OrderedFloat;
use std::f32;
use yoga::{
    Align, Context, Direction, FlexDirection as YogaFlexDirection, FlexStyle, MeasureMode,
    Node as YogaNode, NodeRef, StyleUnit, Wrap,
};

use super::LayoutService;
use crate::api::{
    ComputedLayout, Dimension, Flex, FlexAlign, FlexDirection, FlexWrap, Flow, JustifyContent,
    Rect, Scene, Size, SurfaceId, Text,
};
use crate::text::{PangoService, TextMeasurer};
use crate::Id;
use yoga::types::Justify;

pub struct YogaLayoutService {
    yoga_nodes: Vec<YogaNode>,
    pango_service: PangoService,
}

impl<'svc> YogaLayoutService {
    pub fn new() -> Self {
        YogaLayoutService {
            yoga_nodes: vec![],
            pango_service: PangoService::new(),
        }
    }

    pub fn alloc(&mut self) {
        self.yoga_nodes.push(YogaNode::new())
    }

    pub fn append_child(&mut self, parent: Id, child: Id) {
        let (parent, child) = get_two_muts(&mut self.yoga_nodes, parent, child);

        let index = parent.get_child_count();
        parent.insert_child(child, index);
    }

    pub fn remove_child(&mut self, parent: Id, child: Id) {
        let (parent, child) = get_two_muts(&mut self.yoga_nodes, parent, child);

        parent.remove_child(child);
    }

    // easier with index rather than with Id
    pub fn insert_at(&mut self, parent: Id, child: Id, index: u32) {
        let (parent, child) = get_two_muts(&mut self.yoga_nodes, parent, child);

        parent.insert_child(child, index);
    }

    pub fn set_size(&mut self, id: Id, size: Size) {
        self.yoga_nodes[id].apply_styles(&vec![
            FlexStyle::Width(size.0.into()),
            FlexStyle::Height(size.1.into()),
        ])
    }

    pub fn set_flex(&mut self, id: Id, flex: Flex) {
        self.yoga_nodes[id].apply_styles(&vec![
            FlexStyle::FlexGrow(flex.flex_grow.into()),
            FlexStyle::FlexShrink(flex.flex_shrink.into()),
            FlexStyle::FlexBasis(flex.flex_basis.into()),
        ]);
    }

    pub fn set_flow(&mut self, id: Id, flow: Flow) {
        self.yoga_nodes[id].apply_styles(&vec![
            FlexStyle::FlexDirection(flow.flex_direction.into()),
            FlexStyle::FlexWrap(flow.flex_wrap.into()),
            FlexStyle::JustifyContent(flow.justify_content.into()),
            FlexStyle::AlignContent(flow.align_content.into()),
            FlexStyle::AlignItems(flow.align_items.into()),
        ]);
    }

    pub fn set_padding(&mut self, id: Id, padding: Rect) {
        self.yoga_nodes[id].apply_styles(&vec![
            FlexStyle::PaddingTop(padding.0.into()),
            FlexStyle::PaddingRight(padding.1.into()),
            FlexStyle::PaddingBottom(padding.2.into()),
            FlexStyle::PaddingLeft(padding.3.into()),
        ]);
    }

    pub fn set_margin(&mut self, id: Id, margin: Rect) {
        self.yoga_nodes[id].apply_styles(&vec![
            FlexStyle::MarginTop(margin.0.into()),
            FlexStyle::MarginRight(margin.1.into()),
            FlexStyle::MarginBottom(margin.2.into()),
            FlexStyle::MarginLeft(margin.3.into()),
        ]);
    }

    pub fn set_text(&'svc mut self, id: Id, text: Option<Text>) {
        let node = &mut self.yoga_nodes[id];

        if let Some(text) = text {
            // TODO: this should be done better
            // needed because yoga context has static lifetime
            // yoga context is dropped but Box<Any> does not know which destructor to call
            // so the memory will be freed but no destructors will be called, actually
            // which is not an issue right now but it's certainly not a good way
            let text_measurer: &'static PangoService =
                unsafe { std::mem::transmute(&self.pango_service) };

            node.set_measure_func(Some(measure_text_node));
            node.set_context(Some(Context::new(MeasureContext {
                text_measurer,
                text,
            })));

            node.mark_dirty();
        } else {
            node.set_measure_func(None);
            node.set_context(None);
        }
    }
}

impl LayoutService for YogaLayoutService {
    fn get_computed_layouts(&mut self, surface: SurfaceId) -> Vec<ComputedLayout> {
        self.yoga_nodes[surface].calculate_layout(f32::MAX, f32::MAX, Direction::LTR);

        self.yoga_nodes
            .iter()
            .map(|n| {
                (
                    n.get_layout_left(),
                    n.get_layout_top(),
                    n.get_layout_width(),
                    n.get_layout_height(),
                )
            })
            .collect()
    }
}

extern "C" fn measure_text_node(
    node_ref: NodeRef,
    w: f32,
    wm: MeasureMode,
    _h: f32,
    _hm: MeasureMode,
) -> yoga::Size {
    let ctx = YogaNode::get_context(&node_ref).expect("no context found");
    let ctx = ctx
        .downcast_ref::<MeasureContext>()
        .expect("not a measure context");

    let max_width = match wm {
        MeasureMode::Exactly => Some(w),
        MeasureMode::AtMost => Some(w),
        MeasureMode::Undefined => None,
    };

    let (width, height) = ctx.text_measurer.measure_text(&ctx.text, max_width);

    let width = match wm {
        MeasureMode::Exactly => w,
        MeasureMode::AtMost => width,
        MeasureMode::Undefined => width,
    };

    let size = yoga::Size { width, height };

    debug!("measure {:?}", (&ctx.text.text, w, wm, &size));

    size
}

struct MeasureContext<'svc> {
    pub text_measurer: &'svc PangoService,
    pub text: Text,
}

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
