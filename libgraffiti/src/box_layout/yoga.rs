use crate::commons::{SurfaceId, Pos, Bounds};
use super::{BoxLayout, BoxLayoutNode, Display, Dimension, Align, FlexDirection, FlexWrap};
use crate::text_layout::{TextLayout};
use graffiti_yoga::*;

pub struct YogaLayout {
    window_size: (f32, f32),
    yoga_nodes: Vec<YGNodeRef>,
    text_layout: *mut TextLayout,
    bounds: Vec<Bounds>,
}

impl YogaLayout {
    pub fn new(width: i32, height: i32, text_layout: *mut TextLayout) -> Self {
        let mut res = YogaLayout {
            window_size: (0., 0.),
            yoga_nodes: Vec::new(),
            text_layout,
            bounds: Vec::new(),
        };

        res.alloc();
        res.resize(width, height);

        res
    }
}

impl BoxLayout<YGNodeRef> for YogaLayout {
    fn alloc(&mut self) {
        let mut node = unsafe { YGNodeNew() };

        node.set_flex_direction(FlexDirection::Row);
        node.set_flex_basis(Dimension::Auto);
        node.set_flex_shrink(1.);

        self.yoga_nodes.push(node);
        self.bounds.push(Bounds::zero());
    }

    fn insert_at(&mut self, parent: SurfaceId, child: SurfaceId, index: usize) {
        unsafe {
            YGNodeInsertChild(self.yoga_nodes[parent], self.yoga_nodes[child], index as u32);
        }
    }

    fn remove_child(&mut self, parent: SurfaceId, child: SurfaceId) {
        unsafe {
            YGNodeRemoveChild(self.yoga_nodes[parent], self.yoga_nodes[child]);
        }
    }

    fn get_node_mut(&mut self, id: SurfaceId) -> &mut YGNodeRef {
        &mut self.yoga_nodes[id]
    }

    // TODO: this is temporary
    fn set_measure_text(&mut self, id: SurfaceId, measure: bool) {
        let node = self.yoga_nodes[id];

        unsafe {
            if measure {
                YGNodeSetMeasureFunc(node, Some(measure_text_node));
                YGNodeMarkDirty(node);
                YGNodeSetContext(node, Box::into_raw(Box::new(MeasureContext(id, self.text_layout))) as *mut std::os::raw::c_void);
            } else {
                YGNodeSetMeasureFunc(node, None);
                YGNodeSetContext(node, std::ptr::null_mut());

                // TODO: free memory
            }
        }
    }

    fn calculate(&mut self) {
        unsafe {
            YGNodeCalculateLayout(self.yoga_nodes[0], self.window_size.0, YGUndefined, YGDirection::LTR);

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
            YGNodeStyleSetMinHeight(root, size.1);
        }

        self.window_size = size;
    }

    fn get_bounds(&self) -> &[Bounds] {
        &self.bounds
    }
}

// generate setters
// unfortunately, macros can't prefix idents
// $set_auto is optional
macro_rules! dim_setter {
    ($meth:ident $set:ident $set_perc:ident $($set_auto:ident)*) => (
        #[allow(unreachable_patterns)]
        fn $meth(&mut self, v: Dimension) {
            unsafe {
                match v {
                    Dimension::Px { value } => $set(*self, value),
                    Dimension::Percent { value } => $set_perc(*self, value),
                    $(Dimension::Auto => $set_auto(*self),)*
                    Dimension::Undefined => $set(*self, YGUndefined),
                    _ => {
                        error!("unexpected {:?} {:?}", stringify!($meth), &v);
                    }
                }
            }
        }
    )
}

// DRY later
// but edge is both a prefix & suffix which might be challenge
macro_rules! dim_edge_setter {
    ($meth:ident $edge:ident $set:ident $set_perc:ident $($set_auto:ident)*) => (
        #[allow(unreachable_patterns)]
        fn $meth(&mut self, v: Dimension) {
            unsafe {
                match v {
                    Dimension::Px { value } => $set(*self, YGEdge::$edge, value),
                    Dimension::Percent { value } => $set_perc(*self, YGEdge::$edge, value),
                    $(Dimension::Auto => $set_auto(*self, YGEdge::$edge),)*
                    Dimension::Undefined => $set(*self, YGEdge::$edge, YGUndefined),
                    _ => {
                        error!("unexpected {:?} {:?}", stringify!($meth), &v);
                    }
                }
            }
        }
    )
}

impl BoxLayoutNode for *mut YGNode {
    fn set_display(&mut self, _v: Display) {
        silly!("TODO: set_display")
    }

    dim_setter!(set_width YGNodeStyleSetWidth YGNodeStyleSetWidthPercent YGNodeStyleSetWidthAuto);
    dim_setter!(set_height YGNodeStyleSetHeight YGNodeStyleSetHeightPercent YGNodeStyleSetHeightAuto);
    dim_setter!(set_min_width YGNodeStyleSetMinWidth YGNodeStyleSetWidthPercent);
    dim_setter!(set_min_height YGNodeStyleSetMinHeight YGNodeStyleSetHeightPercent);
    dim_setter!(set_max_width YGNodeStyleSetMaxWidth YGNodeStyleSetMaxWidthPercent);
    dim_setter!(set_max_height YGNodeStyleSetMaxHeight YGNodeStyleSetMaxHeightPercent);

    dim_edge_setter!(set_top Top YGNodeStyleSetPosition YGNodeStyleSetPositionPercent);
    dim_edge_setter!(set_right Right YGNodeStyleSetPosition YGNodeStyleSetPositionPercent);
    dim_edge_setter!(set_bottom Bottom YGNodeStyleSetPosition YGNodeStyleSetPositionPercent);
    dim_edge_setter!(set_left Left YGNodeStyleSetPosition YGNodeStyleSetPositionPercent);

    dim_edge_setter!(set_margin_top Top YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto);
    dim_edge_setter!(set_margin_right Right YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto);
    dim_edge_setter!(set_margin_bottom Bottom YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto);
    dim_edge_setter!(set_margin_left Left YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto);

    dim_edge_setter!(set_padding_top Top YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent);
    dim_edge_setter!(set_padding_right Right YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent);
    dim_edge_setter!(set_padding_bottom Bottom YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent);
    dim_edge_setter!(set_padding_left Left YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent);

    fn set_border_top(&mut self, v: f32) {
        unsafe { YGNodeStyleSetBorder(*self, YGEdge::Top, v) }
    }

    fn set_border_right(&mut self, v: f32) {
        unsafe { YGNodeStyleSetBorder(*self, YGEdge::Right, v) }
    }

    fn set_border_bottom(&mut self, v: f32) {
        unsafe { YGNodeStyleSetBorder(*self, YGEdge::Bottom, v) }
    }

    fn set_border_left(&mut self, v: f32) {
        unsafe { YGNodeStyleSetBorder(*self, YGEdge::Left, v) }
    }

    fn set_flex_grow(&mut self, v: f32) {
        unsafe { YGNodeStyleSetFlexGrow(*self, v) }
    }

    fn set_flex_shrink(&mut self, v: f32) {
        unsafe { YGNodeStyleSetFlexShrink(*self, v) }
    }

    dim_setter!(set_flex_basis YGNodeStyleSetFlexBasis YGNodeStyleSetFlexBasisPercent YGNodeStyleSetFlexBasisAuto);

    fn set_flex_direction(&mut self, v: FlexDirection) {
        unsafe { YGNodeStyleSetFlexDirection(*self, v.into()) }        
    }

    fn set_flex_wrap(&mut self, v: FlexWrap) {
        unsafe { YGNodeStyleSetFlexWrap(*self, v.into()) }
    }

    fn set_align_self(&mut self, v: Align) {
        unsafe { YGNodeStyleSetAlignSelf(*self, v.into()) }
    }

    fn set_align_content(&mut self, v: Align) {
        unsafe { YGNodeStyleSetAlignContent(*self, v.into()) }
    }

    fn set_align_items(&mut self, v: Align) {
        unsafe { YGNodeStyleSetAlignItems(*self, v.into()) }
    }

    fn set_justify_content(&mut self, v: Align) {
        unsafe { YGNodeStyleSetJustifyContent(*self, v.into()) }
    }
}

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

unsafe extern "C" fn measure_text_node(
    node: YGNodeRef,
    w: f32,
    wm: YGMeasureMode,
    _h: f32,
    _hm: YGMeasureMode,
) -> YGSize {
    let ctx = YGNodeGetContext(node) as *mut MeasureContext;

    let MeasureContext(id, text_layout) = *ctx;

    debug!("measure {}", id);

    let max_width = match wm {
        YGMeasureMode::Exactly => w,
        YGMeasureMode::AtMost => w,
        YGMeasureMode::Undefined => std::f32::MAX,
    };

    let size = (*text_layout).wrap(id, max_width);

    debug!("result {:?}", &size);

    let width = match wm {
        YGMeasureMode::Exactly => w,
        YGMeasureMode::AtMost => size.0,
        YGMeasureMode::Undefined => size.0,
    };

    YGSize { width, height: size.1 }
}

struct MeasureContext (
    SurfaceId,
    *mut TextLayout
);
