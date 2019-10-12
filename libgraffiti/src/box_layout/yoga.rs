use std::f32;
use crate::commons::{SurfaceId, Pos, Bounds, Border};
use crate::text_layout::{Text};
use yoga::{
    Align as YogaAlign, Context, Direction, FlexStyle, MeasureMode,
    Node as YogaNode, NodeRef, StyleUnit,
    FlexDirection as YogaFlexDirection,
    Wrap, Justify
};

use super::{BoxLayout, DimensionProp, Dimension, AlignProp, Align, FlexDirection, FlexWrap};

type Id = SurfaceId;

pub struct YogaLayout {
    window_size: (f32, f32),
    yoga_nodes: Vec<YogaNode>,
    measure_text_holder: Option<&'static mut dyn FnMut(SurfaceId, f32) -> (f32, f32)>,
    bounds: Vec<Bounds>,
}

impl YogaLayout {
    pub fn new(width: i32, height: i32) -> Self {
        let mut res = YogaLayout {
            window_size: (0., 0.),
            yoga_nodes: Vec::new(),
            measure_text_holder: None,
            bounds: Vec::new(),
        };

        res.alloc();
        res.resize(width, height);

        res
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
}

impl BoxLayout for YogaLayout {
    fn alloc(&mut self) {
        self.yoga_nodes.push(YogaNode::new());
        self.bounds.push(Bounds::zero());
    }

    fn insert_at(&mut self, parent: Id, child: Id, index: usize) {
        let (parent, child) = get_two_muts(&mut self.yoga_nodes, parent, child);

        parent.insert_child(child, index as u32);
    }

    fn remove_child(&mut self, parent: Id, child: Id) {
        let (parent, child) = get_two_muts(&mut self.yoga_nodes, parent, child);

        parent.remove_child(child);
    }

    fn set_dimension(&mut self, surface: SurfaceId, prop: DimensionProp, value: Dimension) {
        let v = value.into();

        self.yoga_nodes[surface].apply_style(&match prop {
            DimensionProp::Width => FlexStyle::Width(v),
            DimensionProp::Height => FlexStyle::Height(v),
            DimensionProp::MinWidth => FlexStyle::MinWidth(v),
            DimensionProp::MinHeight => FlexStyle::MinHeight(v),
            DimensionProp::MaxWidth => FlexStyle::MaxWidth(v),
            DimensionProp::MaxHeight => FlexStyle::MaxHeight(v),

            DimensionProp::PaddingLeft => FlexStyle::PaddingLeft(v),
            DimensionProp::PaddingRight => FlexStyle::PaddingRight(v),
            DimensionProp::PaddingTop => FlexStyle::PaddingTop(v),
            DimensionProp::PaddingBottom => FlexStyle::PaddingBottom(v),

            DimensionProp::MarginLeft => FlexStyle::MarginLeft(v),
            DimensionProp::MarginRight => FlexStyle::MarginRight(v),
            DimensionProp::MarginTop => FlexStyle::MarginTop(v),
            DimensionProp::MarginBottom => FlexStyle::MarginBottom(v),

            DimensionProp::FlexGrow => FlexStyle::FlexGrow(get_points(&v).into()),
            DimensionProp::FlexShrink => FlexStyle::FlexShrink(get_points(&v).into()),
            DimensionProp::FlexBasis => FlexStyle::FlexBasis(v),
        })
    }

    fn set_align(&mut self, surface: SurfaceId, prop: AlignProp, value: Align) {
        self.yoga_nodes[surface].apply_style(&match prop {
            AlignProp::AlignSelf => FlexStyle::AlignSelf(value.into()),
            AlignProp::AlignContent => FlexStyle::AlignContent(value.into()),
            AlignProp::AlignItems => FlexStyle::AlignItems(value.into()),
            AlignProp::JustifyContent => FlexStyle::JustifyContent(value.into()),
        })
    }

    fn set_flex_direction(&mut self, surface: SurfaceId, value: FlexDirection) {
        self.yoga_nodes[surface].apply_style(&FlexStyle::FlexDirection(value.into()));
    }

    fn set_flex_wrap(&mut self, surface: SurfaceId, value: FlexWrap) {
        self.yoga_nodes[surface].apply_style(&FlexStyle::FlexWrap(value.into()));
    }

    // separate because rendering doesn't need to test dimensions then
    fn set_border(&mut self, surface: SurfaceId, border: Option<Border>) {
        YogaLayout::set_border(self, surface, border);
    }

    // another separate
    fn set_text(&mut self, surface: SurfaceId, text: Option<Text>) {
        YogaLayout::set_text(self, surface, text);
    }

    fn calculate(&mut self, measure_text: &mut dyn FnMut(SurfaceId, f32) -> (f32, f32)) {
        self.measure_text_holder = Some(unsafe { std::mem::transmute(measure_text) });
        self.yoga_nodes[0].calculate_layout(self.window_size.0, self.window_size.1, Direction::LTR);
        self.measure_text_holder = None;

        // TODO: update only attached and display != none nodes
        for i in 0..self.yoga_nodes.len() {
            let n = &mut self.yoga_nodes[i];
            let a = Pos::new(n.get_layout_left(), n.get_layout_top());
            let b = Pos::new(n.get_layout_left() + n.get_layout_width(), n.get_layout_top() + n.get_layout_height());

            self.bounds[i] = Bounds { a, b };
        }
    }

    fn resize(&mut self, width: i32, height: i32) {
        let size = (width as f32, height as f32);
        let root = &mut self.yoga_nodes[0];

        root.set_width(StyleUnit::Point(size.0.into()));
        root.set_height(StyleUnit::Point(size.1.into()));

        self.window_size = size;
    }

    fn get_bounds(&self) -> &[Bounds] {
        &self.bounds
    }

    /*
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
    */
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
        MeasureMode::Exactly => w,
        MeasureMode::AtMost => w,
        MeasureMode::Undefined => std::f32::MAX,
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
            Dimension { point: None, percent: None } => StyleUnit::Auto,
            Dimension { point: Some(p), .. } => StyleUnit::Point(p.into()),
            Dimension { percent: Some(p), .. } => StyleUnit::Percent(p.into()),
        }
    }
}

impl Into<YogaAlign> for Align {
    fn into(self) -> YogaAlign {
        match self {
            Align::Auto => YogaAlign::Auto,
            Align::Baseline => YogaAlign::Baseline,
            Align::Center => YogaAlign::Center,
            Align::FlexStart => YogaAlign::FlexStart,
            Align::FlexEnd => YogaAlign::FlexEnd,
            Align::SpaceAround => YogaAlign::SpaceAround,
            Align::SpaceBetween => YogaAlign::SpaceBetween,
            Align::Stretch => YogaAlign::Stretch,
            _ => panic!("invalid align")
        }
    }
}

impl Into<Justify> for Align {
    fn into(self) -> Justify {
        match self {
            Align::Center => Justify::Center,
            Align::FlexStart => Justify::FlexStart,
            Align::FlexEnd => Justify::FlexEnd,
            Align::SpaceAround => Justify::SpaceAround,
            Align::SpaceBetween => Justify::SpaceBetween,
            Align::SpaceEvenly => Justify::SpaceEvenly,
            _ => panic!("invalid justify")
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

impl Into<Wrap> for FlexWrap {
    fn into(self) -> Wrap {
        match self {
            FlexWrap::Wrap => Wrap::Wrap,
            FlexWrap::WrapReverse => Wrap::WrapReverse,
            FlexWrap::NoWrap => Wrap::NoWrap,
        }
    }
}

/*
impl Into<yoga::Overflow> for Overflow {
    fn into(self) -> yoga::Overflow {
        match self {
            Overflow::Visible => yoga::Overflow::Visible,
            Overflow::Hidden => yoga::Overflow::Hidden,
            Overflow::Scroll => yoga::Overflow::Scroll
        }
    }
}
*/

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

// hacky because of into(), type inference & DRY :-/
fn get_points(dim: &StyleUnit) -> f32 {
    match dim {
        StyleUnit::Point(v) => **v,
        _ => panic!("expected point")
    }
}
