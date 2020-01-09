use crate::commons::{SurfaceId, Pos, Bounds};
use super::{BoxLayoutTree, Display, Dimension, Align, FlexDirection, FlexWrap};
use graffiti_yoga::*;

// this impl. is not generic (the trait is)
type NodeId = usize;
type MeasureKey = usize;

pub struct YogaLayoutTree {
    window_size: (f32, f32),
    yoga_nodes: Vec<YGNodeRef>,
    bounds: Vec<Bounds>,
}

impl YogaLayoutTree {
    pub fn new(width: i32, height: i32) -> Self {
        let mut res = YogaLayoutTree {
            window_size: (0., 0.),
            yoga_nodes: Vec::new(),
            bounds: Vec::new(),
        };

        res.create_node();
        res.resize(width, height);

        res
    }
}

// generate setters
// unfortunately, macros can't prefix idents
// $set_auto is optional
macro_rules! dim_setter {
    ($meth:ident $set:ident $set_perc:ident $($set_auto:ident)*) => (
        #[allow(unreachable_patterns)]
        fn $meth(&mut self, node: NodeId, v: Dimension) {
            let node = self.yoga_nodes[node];

            unsafe {
                match v {
                    Dimension::Px { value } => $set(node, value),
                    Dimension::Percent { value } => $set_perc(node, value),
                    $(Dimension::Auto => $set_auto(node),)*
                    Dimension::Undefined => $set(node, YGUndefined),
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
        fn $meth(&mut self, node: NodeId, v: Dimension) {
            let node = self.yoga_nodes[node];

            unsafe {
                match v {
                    Dimension::Px { value } => $set(node, YGEdge::$edge, value),
                    Dimension::Percent { value } => $set_perc(node, YGEdge::$edge, value),
                    $(Dimension::Auto => $set_auto(node, YGEdge::$edge),)*
                    Dimension::Undefined => $set(node, YGEdge::$edge, YGUndefined),
                    _ => {
                        error!("unexpected {:?} {:?}", stringify!($meth), &v);
                    }
                }
            }
        }
    )
}

impl BoxLayoutTree<NodeId, MeasureKey> for YogaLayoutTree {
    fn create_node(&mut self) -> NodeId {
        let id = self.yoga_nodes.len();
        let node = unsafe { YGNodeNew() };

        self.yoga_nodes.push(node);
        self.bounds.push(Bounds::zero());

        self.set_flex_direction(id, FlexDirection::Row);
        self.set_flex_basis(id, Dimension::Auto);
        self.set_flex_shrink(id, 1.);

        id
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

    fn calculate(&mut self, measure_fn: &mut dyn FnMut(MeasureKey, f32) -> (f32, f32)) {
        unsafe {
            if MEASURE_REF.is_some() {
                panic!("layout not thread-safe");
            }

            MEASURE_REF = Some(std::mem::transmute(measure_fn));

            YGNodeCalculateLayout(self.yoga_nodes[0], self.window_size.0, YGUndefined, YGDirection::LTR);

            // TODO: update only attached and display != none nodes
            for i in 0..self.yoga_nodes.len() {
                let n = self.yoga_nodes[i];
                let a = Pos::new(YGNodeLayoutGetLeft(n), YGNodeLayoutGetTop(n));
                let b = Pos::new(YGNodeLayoutGetLeft(n) + YGNodeLayoutGetWidth(n), YGNodeLayoutGetTop(n) + YGNodeLayoutGetHeight(n));

                self.bounds[i] = Bounds { a, b };
            }

            MEASURE_REF = None;
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

    fn set_display(&mut self, node: NodeId, _v: Display) {
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

    fn set_border_top(&mut self, node: NodeId, v: f32) {
        unsafe { YGNodeStyleSetBorder(self.yoga_nodes[node], YGEdge::Top, v) }
    }

    fn set_border_right(&mut self, node: NodeId, v: f32) {
        unsafe { YGNodeStyleSetBorder(self.yoga_nodes[node], YGEdge::Right, v) }
    }

    fn set_border_bottom(&mut self, node: NodeId, v: f32) {
        unsafe { YGNodeStyleSetBorder(self.yoga_nodes[node], YGEdge::Bottom, v) }
    }

    fn set_border_left(&mut self, node: NodeId, v: f32) {
        unsafe { YGNodeStyleSetBorder(self.yoga_nodes[node], YGEdge::Left, v) }
    }

    fn set_flex_grow(&mut self, node: NodeId, v: f32) {
        unsafe { YGNodeStyleSetFlexGrow(self.yoga_nodes[node], v) }
    }

    fn set_flex_shrink(&mut self, node: NodeId, v: f32) {
        unsafe { YGNodeStyleSetFlexShrink(self.yoga_nodes[node], v) }
    }

    dim_setter!(set_flex_basis YGNodeStyleSetFlexBasis YGNodeStyleSetFlexBasisPercent YGNodeStyleSetFlexBasisAuto);

    fn set_flex_direction(&mut self, node: NodeId, v: FlexDirection) {
        unsafe { YGNodeStyleSetFlexDirection(self.yoga_nodes[node], v.into()) }        
    }

    fn set_flex_wrap(&mut self, node: NodeId, v: FlexWrap) {
        unsafe { YGNodeStyleSetFlexWrap(self.yoga_nodes[node], v.into()) }
    }

    fn set_align_self(&mut self, node: NodeId, v: Align) {
        unsafe { YGNodeStyleSetAlignSelf(self.yoga_nodes[node], v.into()) }
    }

    fn set_align_content(&mut self, node: NodeId, v: Align) {
        unsafe { YGNodeStyleSetAlignContent(self.yoga_nodes[node], v.into()) }
    }

    fn set_align_items(&mut self, node: NodeId, v: Align) {
        unsafe { YGNodeStyleSetAlignItems(self.yoga_nodes[node], v.into()) }
    }

    fn set_justify_content(&mut self, node: NodeId, v: Align) {
        unsafe { YGNodeStyleSetJustifyContent(self.yoga_nodes[node], v.into()) }
    }

    fn set_measure_key(&mut self, node: NodeId, key: Option<MeasureKey>) {
        let node = self.yoga_nodes[node];

        unsafe {
            match key {
                None => {
                    YGNodeSetMeasureFunc(node, None);
                    YGNodeSetContext(node, std::ptr::null_mut());

                    // TODO: free memory
                },
                Some(key) => {
                    YGNodeSetMeasureFunc(node, Some(measure_text_node));
                    YGNodeMarkDirty(node);
                    YGNodeSetContext(node, std::mem::transmute(std::boxed::Box::<MeasureKey>::leak(Box::new(key))));
                }
            }
        }
    }
}

static mut MEASURE_REF: Option<&'static mut dyn FnMut(MeasureKey, f32) -> (f32, f32)> = None;

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
    let measure = MEASURE_REF.as_mut().expect("measure not set");
    let key = *(YGNodeGetContext(node) as *mut MeasureKey);

    let max_width = match wm {
        YGMeasureMode::Exactly => w,
        YGMeasureMode::AtMost => w,
        YGMeasureMode::Undefined => std::f32::MAX,
    };

    let size = measure(key, max_width);

    let width = match wm {
        YGMeasureMode::Exactly => w,
        YGMeasureMode::AtMost => size.0,
        YGMeasureMode::Undefined => size.0,
    };

    YGSize { width, height: size.1 }
}
