#[cfg(test)]
use ordered_float::OrderedFloat;
use std::f32;
use yoga::{
    Align, Context, Direction, FlexDirection as YogaFlexDirection, FlexStyle, MeasureMode,
    Node as YogaNode, NodeRef, StyleUnit, Wrap,
};

use super::Layout;
use crate::generated::{Border, Dimension, Dimensions, Flex, FlexAlign, FlexDirection, FlexWrap, Flow, JustifyContent, Overflow, Rect, Size, Text, SurfaceId, UpdateSceneMsg, StyleProp, Vector2f};
use yoga::types::Justify;
use crate::SceneListener;

type Id = SurfaceId;

pub struct YogaLayout {
    yoga_nodes: Vec<YogaNode>,
    measure_text_holder: Option<&'static mut dyn FnMut(SurfaceId, Option<f32>) -> Vector2f>
}

impl SceneListener for YogaLayout {
    fn update_scene(&mut self, msgs: &[UpdateSceneMsg]) {
        for m in msgs.iter().cloned() {
            match m {
                UpdateSceneMsg::Alloc => self.alloc(),
                UpdateSceneMsg::InsertAt { parent, child, index } => self.insert_at(parent, child, index as u32),
                UpdateSceneMsg::RemoveChild { parent, child } => self.remove_child(parent, child),
                UpdateSceneMsg::SetStyleProp { surface, prop } => {
                    match prop {
                        StyleProp::Size(s) => self.set_size(surface, s),
                        StyleProp::Flex(f) => self.set_flex(surface, f),
                        StyleProp::Flow(f) => self.set_flow(surface, f),
                        StyleProp::Padding(p) => self.set_padding(surface, p),
                        StyleProp::Border(b) => self.set_border(surface, b),
                        StyleProp::Margin(m) => self.set_margin(surface, m),
                        StyleProp::Text(t) => self.set_text(surface, t),
                        StyleProp::Overflow(o) => self.set_overflow(surface, o),
                        _ => {}
                    }
                }
            }
        }
    }
}

impl YogaLayout {
    pub fn new() -> Self {
        YogaLayout {
            yoga_nodes: vec![YogaNode::new()],
            measure_text_holder: None
        }
    }

    fn alloc(&mut self) {
        self.yoga_nodes.push(YogaNode::new())
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
            FlexStyle::AlignSelf(flow.align_self.into()),
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

    fn set_border(&mut self, id: Id, border: Option<Border>) {
        let widths = border.map_or([0., 0., 0., 0.], |b| [b.top.width, b.right.width, b.bottom.width, b.left.width]);

        self.yoga_nodes[id].apply_styles(&[
            FlexStyle::BorderTop(widths[0].into()),
            FlexStyle::BorderRight(widths[1].into()),
            FlexStyle::BorderBottom(widths[2].into()),
            FlexStyle::BorderLeft(widths[3].into())
        ])
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
        let self_ref = get_static_ref(self);
        let node = &mut self.yoga_nodes[id];

        if text.is_some() {
            node.set_measure_func(Some(measure_text_node));
            node.mark_dirty();
            node.set_context(Some(Context::new(MeasureContext(id, self_ref))));
        } else {
            node.set_measure_func(None);
            node.set_context(None);
        }
    }

    fn set_overflow(&mut self, id: Id, overflow: Overflow) {
        self.yoga_nodes[id].set_overflow(overflow.into());
    }
}

impl Layout for YogaLayout {
    fn calculate(&mut self, measure_text: &mut dyn FnMut(SurfaceId, Option<f32>) -> (f32, f32)) {
        self.measure_text_holder = Some(unsafe { std::mem::transmute(measure_text) });
        self.yoga_nodes[0].calculate_layout(f32::MAX, f32::MAX, Direction::LTR);
        self.measure_text_holder = None;
    }

    fn get_rect(&self, id: SurfaceId) -> Rect {
        let n = &self.yoga_nodes[id];

        Rect(
            n.get_layout_left(),
            n.get_layout_top(),
            n.get_layout_width(),
            n.get_layout_height()
        )
    }

    fn get_scroll_frame(&self, id: SurfaceId) -> Option<(f32, f32)> {
        let node = &self.yoga_nodes[id];

        match node.get_overflow() {
            yoga::Overflow::Scroll => match node.get_child_count() {
                1 => {
                    let child: YogaNode = unsafe { std::mem::transmute(node.get_child(0)) };
                    let width = child.get_layout_width() + node.get_layout_padding_left() + node.get_layout_padding_right();
                    let height = child.get_layout_height() + node.get_layout_padding_top() + node.get_layout_padding_bottom();
                    std::mem::forget(child);

                    Some((width, height))
                },
                // it shouldn't be that hard but it's not on the list
                _ => unimplemented!("for now we only support overflow: 'scroll' for ScrollView which always has one child")
            },
            _ => None
        }
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
    let MeasureContext(id, yoga_layout) = ctx
        .downcast_mut::<MeasureContext>()
        .expect("not a measure context");

    let measure_text = yoga_layout.measure_text_holder.as_mut().expect("missing measure_text fn");

    let max_width = match wm {
        MeasureMode::Exactly => Some(w),
        MeasureMode::AtMost => Some(w),
        MeasureMode::Undefined => None,
    };

    let size = measure_text(*id, max_width);

    let width = match wm {
        MeasureMode::Exactly => w,
        MeasureMode::AtMost => size.0,
        MeasureMode::Undefined => size.0,
    };

    yoga::Size { width, height: size.1 }
}

struct MeasureContext<'a> (
    pub Id,
    pub &'a mut YogaLayout
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

impl Into<yoga::Overflow> for Overflow {
    fn into(self) -> yoga::Overflow {
        match self {
            Overflow::Visible => yoga::Overflow::Visible,
            Overflow::Hidden => yoga::Overflow::Hidden,
            Overflow::Scroll => yoga::Overflow::Scroll
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

pub fn get_static_ref(yoga_layout: &mut YogaLayout) -> &'static mut YogaLayout {
    unsafe { std::mem::transmute(yoga_layout) }
}
