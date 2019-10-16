use std::f32;
use crate::commons::{SurfaceId, Pos, Bounds, Border};
use crate::text_layout::{Text};
use graffiti_yoga::*;

use super::{BoxLayout, DimensionProp, Dimension, AlignProp, Align, FlexDirection, FlexWrap};

type Id = SurfaceId;

pub struct YogaLayout {
    window_size: (f32, f32),
    yoga_nodes: Vec<YGNodeRef>,
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
        let n = self.yoga_nodes[id];

        unsafe {
            YGNodeStyleSetBorder(n, YGEdge::Top, widths[0]);
            YGNodeStyleSetBorder(n, YGEdge::Right, widths[0]);
            YGNodeStyleSetBorder(n, YGEdge::Bottom, widths[0]);
            YGNodeStyleSetBorder(n, YGEdge::Left, widths[0]);
        }
    }

    fn set_text<'svc>(&mut self, id: Id, text: Option<Text>) {
        let self_ref = get_static_ref(self);
        let node = self.yoga_nodes[id];

        unsafe {
            if text.is_some() {
                YGNodeSetMeasureFunc(node, Some(measure_text_node));
                YGNodeMarkDirty(node);
                YGNodeSetContext(node, Box::into_raw(Box::new(MeasureContext(id, self_ref))) as *mut std::os::raw::c_void);
            } else {
                YGNodeSetMeasureFunc(node, None);
                YGNodeSetContext(node, std::ptr::null_mut());

                // TODO: free memory
            }
        }

    }
}

impl BoxLayout for YogaLayout {
    fn alloc(&mut self) {
        self.yoga_nodes.push(unsafe { YGNodeNew() });
        self.bounds.push(Bounds::zero());
    }

    fn insert_at(&mut self, parent: Id, child: Id, index: usize) {
        let (parent, child) = get_two_muts(&mut self.yoga_nodes, parent, child);

        unsafe {
            YGNodeInsertChild(*parent, *child, index as u32);
        }
    }

    fn remove_child(&mut self, parent: Id, child: Id) {
        let (parent, child) = get_two_muts(&mut self.yoga_nodes, parent, child);

        unsafe {
            YGNodeRemoveChild(*parent, *child);
        }
    }

    fn set_dimension(&mut self, surface: SurfaceId, prop: DimensionProp, value: Dimension) {
        let n = self.yoga_nodes[surface];

        unsafe {
            match value {
                Dimension { point: None, percent: None } => match prop {
                    DimensionProp::Width => YGNodeStyleSetWidthAuto(n),
                    DimensionProp::Height => YGNodeStyleSetHeightAuto(n),

                    DimensionProp::FlexBasis => YGNodeStyleSetFlexBasisAuto(n),

                    DimensionProp::MarginTop => YGNodeStyleSetMarginAuto(n, YGEdge::Top),
                    DimensionProp::MarginRight => YGNodeStyleSetMarginAuto(n, YGEdge::Right),
                    DimensionProp::MarginBottom => YGNodeStyleSetMarginAuto(n, YGEdge::Bottom),
                    DimensionProp::MarginLeft => YGNodeStyleSetMarginAuto(n, YGEdge::Left),

                    _ => {
                        error!("unexpected {:?} {:?}", &prop, &value);
                    }
                },
                Dimension { percent: Some(v), .. } => match prop {
                    DimensionProp::Width => YGNodeStyleSetWidthPercent(n, v),
                    DimensionProp::Height => YGNodeStyleSetHeightPercent(n, v),
                    DimensionProp::MinWidth => YGNodeStyleSetMinWidthPercent(n, v),
                    DimensionProp::MinHeight => YGNodeStyleSetMinHeightPercent(n, v),
                    DimensionProp::MaxWidth => YGNodeStyleSetMaxWidthPercent(n, v),
                    DimensionProp::MaxHeight => YGNodeStyleSetMaxHeightPercent(n, v),

                    DimensionProp::FlexBasis => YGNodeStyleSetFlexBasisPercent(n, v),

                    DimensionProp::MarginTop => YGNodeStyleSetMarginPercent(n, YGEdge::Top, v),
                    DimensionProp::MarginRight => YGNodeStyleSetMarginPercent(n, YGEdge::Right, v),
                    DimensionProp::MarginBottom => YGNodeStyleSetMarginPercent(n, YGEdge::Bottom, v),
                    DimensionProp::MarginLeft => YGNodeStyleSetMarginPercent(n, YGEdge::Left, v),

                    DimensionProp::PaddingTop => YGNodeStyleSetPaddingPercent(n, YGEdge::Top, v),
                    DimensionProp::PaddingRight => YGNodeStyleSetPaddingPercent(n, YGEdge::Right, v),
                    DimensionProp::PaddingBottom => YGNodeStyleSetPaddingPercent(n, YGEdge::Bottom, v),
                    DimensionProp::PaddingLeft => YGNodeStyleSetPaddingPercent(n, YGEdge::Left, v),

                    _ => {
                        error!("unexpected {:?} {:?}", &prop, &value);
                    }
                },
                Dimension { point: v, .. } => {
                    let v = v.unwrap_or(YGUndefined);

                    match prop {
                        DimensionProp::Width => YGNodeStyleSetWidth(n, v),
                        DimensionProp::Height => YGNodeStyleSetHeight(n, v),
                        DimensionProp::MinWidth => YGNodeStyleSetMinWidth(n, v),
                        DimensionProp::MinHeight => YGNodeStyleSetMinHeight(n, v),
                        DimensionProp::MaxWidth => YGNodeStyleSetMaxWidth(n, v),
                        DimensionProp::MaxHeight => YGNodeStyleSetMaxHeight(n, v),

                        DimensionProp::MarginTop => YGNodeStyleSetMargin(n, YGEdge::Top, v),
                        DimensionProp::MarginRight => YGNodeStyleSetMargin(n, YGEdge::Right, v),
                        DimensionProp::MarginBottom => YGNodeStyleSetMargin(n, YGEdge::Bottom, v),
                        DimensionProp::MarginLeft => YGNodeStyleSetMargin(n, YGEdge::Left, v),

                        DimensionProp::PaddingTop => YGNodeStyleSetPadding(n, YGEdge::Top, v),
                        DimensionProp::PaddingRight => YGNodeStyleSetPadding(n, YGEdge::Right, v),
                        DimensionProp::PaddingBottom => YGNodeStyleSetPadding(n, YGEdge::Bottom, v),
                        DimensionProp::PaddingLeft => YGNodeStyleSetPadding(n, YGEdge::Left, v),

                        DimensionProp::FlexGrow => YGNodeStyleSetFlexGrow(n, v),
                        DimensionProp::FlexShrink => YGNodeStyleSetFlexShrink(n, v),
                        DimensionProp::FlexBasis => YGNodeStyleSetFlexBasis(n, v),
                    }
                }
            }
        }
    }

    fn set_align(&mut self, surface: SurfaceId, prop: AlignProp, value: Align) {
        let n = self.yoga_nodes[surface];

        unsafe {
            match prop {
                AlignProp::AlignSelf => YGNodeStyleSetAlignSelf(n, value.into()),
                AlignProp::AlignContent => YGNodeStyleSetAlignContent(n, value.into()),
                AlignProp::AlignItems => YGNodeStyleSetAlignItems(n, value.into()),
                AlignProp::JustifyContent => YGNodeStyleSetJustifyContent(n, value.into()),
            }
        }
    }

    fn set_flex_direction(&mut self, surface: SurfaceId, value: FlexDirection) {
        unsafe {
            YGNodeStyleSetFlexDirection(self.yoga_nodes[surface], value.into());
        }
    }

    fn set_flex_wrap(&mut self, surface: SurfaceId, value: FlexWrap) {
        unsafe {
            YGNodeStyleSetFlexWrap(self.yoga_nodes[surface], value.into());
        }
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

        unsafe {
            YGNodeCalculateLayout(self.yoga_nodes[0], self.window_size.0, self.window_size.1, YGDirection::LTR);

            self.measure_text_holder = None;

            // TODO: update only attached and display != none nodes
            for i in 0..self.yoga_nodes.len() {
                let n = self.yoga_nodes[i];
                let a = Pos::new(YGNodeLayoutGetLeft(n), YGNodeLayoutGetTop(n));
                let b = Pos::new(YGNodeLayoutGetLeft(n) + YGNodeLayoutGetWidth(n), YGNodeLayoutGetTop(n) + YGNodeLayoutGetHeight(n));

                self.bounds[i] = Bounds { a, b };
            }
        }
    }

    fn resize(&mut self, width: i32, height: i32) {
        let size = (width as f32, height as f32);
        let root = self.yoga_nodes[0];

        unsafe {
            YGNodeStyleSetWidth(root, size.0);    
            YGNodeStyleSetHeight(root, size.1);    
        }

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

unsafe extern "C" fn measure_text_node(
    node: YGNodeRef,
    w: f32,
    wm: YGMeasureMode,
    _h: f32,
    _hm: YGMeasureMode,
) -> YGSize {
    let ctx = YGNodeGetContext(node) as *mut MeasureContext;

    let MeasureContext(id, yoga_layout) = &mut *ctx;

    let measure_text = yoga_layout.measure_text_holder.as_mut().expect("missing measure_text fn");

    let max_width = match wm {
        YGMeasureMode::Exactly => w,
        YGMeasureMode::AtMost => w,
        YGMeasureMode::Undefined => std::f32::MAX,
    };

    let size = measure_text(*id, max_width);

    let width = match wm {
        YGMeasureMode::Exactly => w,
        YGMeasureMode::AtMost => size.0,
        YGMeasureMode::Undefined => size.0,
    };

    YGSize { width, height: size.1 }
}

struct MeasureContext<'a> (
    pub Id,
    pub &'a mut YogaLayout
);

impl Into<YGAlign> for Align {
    fn into(self) -> YGAlign {
        match self {
            Align::Auto => YGAlign::Auto,
            Align::Baseline => YGAlign::Baseline,
            Align::Center => YGAlign::Center,
            Align::FlexStart => YGAlign::FlexStart,
            Align::FlexEnd => YGAlign::FlexEnd,
            Align::SpaceAround => YGAlign::SpaceAround,
            Align::SpaceBetween => YGAlign::SpaceBetween,
            Align::Stretch => YGAlign::Stretch,
            _ => panic!("invalid align")
        }
    }
}

impl Into<YGJustify> for Align {
    fn into(self) -> YGJustify {
        match self {
            Align::Center => YGJustify::Center,
            Align::FlexStart => YGJustify::FlexStart,
            Align::FlexEnd => YGJustify::FlexEnd,
            Align::SpaceAround => YGJustify::SpaceAround,
            Align::SpaceBetween => YGJustify::SpaceBetween,
            Align::SpaceEvenly => YGJustify::SpaceEvenly,
            _ => panic!("invalid justify")
        }
    }
}

impl Into<YGFlexDirection> for FlexDirection {
    fn into(self) -> YGFlexDirection {
        match self {
            FlexDirection::Column => YGFlexDirection::Column,
            FlexDirection::ColumnReverse => YGFlexDirection::ColumnReverse,
            FlexDirection::Row => YGFlexDirection::Row,
            FlexDirection::RowReverse => YGFlexDirection::RowReverse,
        }
    }
}

impl Into<YGWrap> for FlexWrap {
    fn into(self) -> YGWrap {
        match self {
            FlexWrap::Wrap => YGWrap::Wrap,
            FlexWrap::WrapReverse => YGWrap::WrapReverse,
            FlexWrap::NoWrap => YGWrap::NoWrap,
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
