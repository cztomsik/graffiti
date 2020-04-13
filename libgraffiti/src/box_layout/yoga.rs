// TODO: generic MeasureKey
// (should be possible, at least we can save pointer in the context
//  and the closure (dyn Fn) is generic so it should know the size of the key)

use super::{Align, BoxLayoutTree, Dimension, Display, FlexDirection, FlexWrap, Overflow};
use crate::commons::{Bounds, Pos};
use graffiti_yoga::*;

pub struct YogaLayoutTree {
    yoga_nodes: Vec<YGNodeRef>,
}

// should be safe (no thread-locals, etc.)
unsafe impl std::marker::Send for YogaLayoutTree {}

impl YogaLayoutTree {
    pub fn new() -> Self {
        YogaLayoutTree { yoga_nodes: Vec::new() }
    }
}

// call various YG* functions depending on the dimension type
// $set_auto is optional
macro_rules! set_dim {
    ($node:ident $value:ident $set:ident $set_perc:ident $($set_auto:ident)*) => (
        #[allow(unreachable_patterns)]
        match $value {
            Dimension::Px(v) => $set($node, v),
            Dimension::Percent(v) => $set_perc($node, v),
            $(Dimension::Auto => $set_auto($node),)*
            Dimension::Undefined => $set($node, YGUndefined),
            _ => {
                error!("unexpected {:?} {:?}", stringify!($meth), &$value);
            }
        }
    )
}

// similar for edge props
macro_rules! set_dim_edge {
    ($node:ident $value:ident $edge:ident $set:ident $set_perc:ident $($set_auto:ident)*) => (
        #[allow(unreachable_patterns)]
        match $value {
            Dimension::Px(v) => $set($node, YGEdge::$edge, v),
            Dimension::Percent(v) => $set_perc($node, YGEdge::$edge, v),
            $(Dimension::Auto => $set_auto($node, YGEdge::$edge),)*
            Dimension::Undefined => $set($node, YGEdge::$edge, YGUndefined),
            _ => {
                error!("unexpected {:?} {:?}", stringify!($meth), &$value);
            }
        }
    )
}

impl BoxLayoutTree for YogaLayoutTree {
    type LayoutNodeId = YGNodeRef;
    type MeasureKey = usize;

    fn create_node(&mut self, measure_key: Option<Self::MeasureKey>) -> YGNodeRef {
        let node = unsafe { YGNodeNew() };

        self.yoga_nodes.push(node);

        // set web defaults
        // TODO: default display should be "inline", we dont support that but flex is wrong too
        self.set_flex_direction(node, FlexDirection::Row);
        self.set_flex_basis(node, Dimension::Auto);
        self.set_flex_shrink(node, 1.);

        if let Some(k) = measure_key {
            unsafe {
                YGNodeSetMeasureFunc(node, Some(measure_text_node));
                YGNodeMarkDirty(node);
                YGNodeSetContext(node, std::boxed::Box::<Self::MeasureKey>::leak(Box::new(k)) as *mut usize as *mut std::ffi::c_void);
            }
        }

        node
    }

    fn insert_child(&mut self, parent: YGNodeRef, index: usize, child: YGNodeRef) {
        unsafe { YGNodeInsertChild(parent, child, index as u32) }
    }

    fn remove_child(&mut self, parent: YGNodeRef, child: YGNodeRef) {
        unsafe { YGNodeRemoveChild(parent, child) }
    }

    fn calculate(&mut self, node: YGNodeRef, size: (f32, f32), measure_fn: &mut dyn FnMut(Self::MeasureKey, f32) -> (f32, f32)) {
        unsafe {
            if MEASURE_REF.is_some() {
                panic!("layout not thread-safe");
            }

            MEASURE_REF = Some(std::mem::transmute(measure_fn));

            // height is ignored (yoga would use it as maxHeight which is not what we want)
            // @see resize() in viewport.rs
            YGNodeCalculateLayout(node, size.0, YGUndefined, YGDirection::LTR);

            MEASURE_REF = None;
        }
    }

    #[inline(always)]
    fn get_bounds(&self, node: YGNodeRef) -> Bounds {
        unsafe {
            let left = YGNodeLayoutGetLeft(node);
            let top = YGNodeLayoutGetTop(node);

            let a = Pos::new(left, top);
            let b = Pos::new(left + YGNodeLayoutGetWidth(node), top + YGNodeLayoutGetHeight(node));

            Bounds { a, b }
        }
    }

    fn set_display(&mut self, node: YGNodeRef, v: Display) {
        // TODO: this is in-complete
        //     display: block works like override but it should keep previously set value
        //     (if it's different from default one)
        //
        // it works only because display: block/flex is usually the first rule
        match v {
            Display::None => todo!("display: none"),
            Display::Block => {
                // wouldn't work when inside flex (<Row>) because it would
                // take whole row and push rest to the side
                // (but that align-items: stretch should be enough in most cases)
                //self.set_width(element, Dimension::Percent { value: 100. });
                self.set_flex_direction(node, FlexDirection::Column);
                self.set_align_items(node, Align::Stretch);
            }
            Display::Flex => {
                self.set_flex_direction(node, FlexDirection::Row);
            }
        }
    }

    fn set_overflow(&mut self, node: YGNodeRef, v: Overflow) {
        unsafe { YGNodeStyleSetOverflow(node, v.into()) }
    }

    fn set_width(&mut self, node: YGNodeRef, v: Dimension) {
        unsafe { set_dim!(node v YGNodeStyleSetWidth YGNodeStyleSetWidthPercent YGNodeStyleSetWidthAuto) }
    }

    fn set_height(&mut self, node: YGNodeRef, v: Dimension) {
        unsafe { set_dim!(node v YGNodeStyleSetHeight YGNodeStyleSetHeightPercent YGNodeStyleSetHeightAuto) }
    }

    fn set_min_width(&mut self, node: YGNodeRef, v: Dimension) {
        unsafe { set_dim!(node v YGNodeStyleSetMinWidth YGNodeStyleSetWidthPercent) }
    }

    fn set_min_height(&mut self, node: YGNodeRef, v: Dimension) {
        unsafe { set_dim!(node v YGNodeStyleSetMinHeight YGNodeStyleSetHeightPercent) }
    }

    fn set_max_width(&mut self, node: YGNodeRef, v: Dimension) {
        unsafe { set_dim!(node v YGNodeStyleSetMaxWidth YGNodeStyleSetMaxWidthPercent) }
    }

    fn set_max_height(&mut self, node: YGNodeRef, v: Dimension) {
        unsafe { set_dim!(node v YGNodeStyleSetMaxHeight YGNodeStyleSetMaxHeightPercent) }
    }

    fn set_top(&mut self, node: YGNodeRef, v: Dimension) {
        unsafe { set_dim_edge!(node v Top YGNodeStyleSetPosition YGNodeStyleSetPositionPercent) }
    }

    fn set_right(&mut self, node: YGNodeRef, v: Dimension) {
        unsafe { set_dim_edge!(node v Right YGNodeStyleSetPosition YGNodeStyleSetPositionPercent) }
    }

    fn set_bottom(&mut self, node: YGNodeRef, v: Dimension) {
        unsafe { set_dim_edge!(node v Bottom YGNodeStyleSetPosition YGNodeStyleSetPositionPercent) }
    }

    fn set_left(&mut self, node: YGNodeRef, v: Dimension) {
        unsafe { set_dim_edge!(node v Left YGNodeStyleSetPosition YGNodeStyleSetPositionPercent) }
    }

    fn set_margin_top(&mut self, node: YGNodeRef, v: Dimension) {
        unsafe { set_dim_edge!(node v Top YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto) }
    }

    fn set_margin_right(&mut self, node: YGNodeRef, v: Dimension) {
        unsafe { set_dim_edge!(node v Right YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto) }
    }

    fn set_margin_bottom(&mut self, node: YGNodeRef, v: Dimension) {
        unsafe { set_dim_edge!(node v Bottom YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto) }
    }

    fn set_margin_left(&mut self, node: YGNodeRef, v: Dimension) {
        unsafe { set_dim_edge!(node v Left YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto) }
    }

    fn set_padding_top(&mut self, node: YGNodeRef, v: Dimension) {
        unsafe { set_dim_edge!(node v Top YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent) }
    }

    fn set_padding_right(&mut self, node: YGNodeRef, v: Dimension) {
        unsafe { set_dim_edge!(node v Right YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent) }
    }

    fn set_padding_bottom(&mut self, node: YGNodeRef, v: Dimension) {
        unsafe { set_dim_edge!(node v Bottom YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent) }
    }

    fn set_padding_left(&mut self, node: YGNodeRef, v: Dimension) {
        unsafe { set_dim_edge!(node v Left YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent) }
    }

    fn set_border_top(&mut self, node: YGNodeRef, v: f32) {
        unsafe { YGNodeStyleSetBorder(node, YGEdge::Top, v) }
    }

    fn set_border_right(&mut self, node: YGNodeRef, v: f32) {
        unsafe { YGNodeStyleSetBorder(node, YGEdge::Right, v) }
    }

    fn set_border_bottom(&mut self, node: YGNodeRef, v: f32) {
        unsafe { YGNodeStyleSetBorder(node, YGEdge::Bottom, v) }
    }

    fn set_border_left(&mut self, node: YGNodeRef, v: f32) {
        unsafe { YGNodeStyleSetBorder(node, YGEdge::Left, v) }
    }

    fn set_flex_grow(&mut self, node: YGNodeRef, v: f32) {
        unsafe { YGNodeStyleSetFlexGrow(node, v) }
    }

    fn set_flex_shrink(&mut self, node: YGNodeRef, v: f32) {
        unsafe { YGNodeStyleSetFlexShrink(node, v) }
    }

    fn set_flex_basis(&mut self, node: YGNodeRef, v: Dimension) {
        unsafe { set_dim!(node v YGNodeStyleSetFlexBasis YGNodeStyleSetFlexBasisPercent YGNodeStyleSetFlexBasisAuto) }
    }

    fn set_flex_direction(&mut self, node: YGNodeRef, v: FlexDirection) {
        unsafe { YGNodeStyleSetFlexDirection(node, v.into()) }
    }

    fn set_flex_wrap(&mut self, node: YGNodeRef, v: FlexWrap) {
        unsafe { YGNodeStyleSetFlexWrap(node, v.into()) }
    }

    fn set_align_self(&mut self, node: YGNodeRef, v: Align) {
        unsafe { YGNodeStyleSetAlignSelf(node, v.into()) }
    }

    fn set_align_content(&mut self, node: YGNodeRef, v: Align) {
        unsafe { YGNodeStyleSetAlignContent(node, v.into()) }
    }

    fn set_align_items(&mut self, node: YGNodeRef, v: Align) {
        unsafe { YGNodeStyleSetAlignItems(node, v.into()) }
    }

    fn set_justify_content(&mut self, node: YGNodeRef, v: Align) {
        unsafe { YGNodeStyleSetJustifyContent(node, v.into()) }
    }

    fn mark_dirty(&mut self, node: YGNodeRef) {
        unsafe { YGNodeMarkDirty(node) }
    }
}

static mut MEASURE_REF: Option<&'static mut dyn FnMut(usize, f32) -> (f32, f32)> = None;

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
            _ => panic!("invalid align"),
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
            _ => panic!("invalid justify"),
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

impl Into<YGOverflow> for Overflow {
    fn into(self) -> YGOverflow {
        match self {
            Overflow::Visible => YGOverflow::Visible,
            Overflow::Hidden => YGOverflow::Hidden,
            Overflow::Scroll => YGOverflow::Scroll,
        }
    }
}

unsafe extern "C" fn measure_text_node(node: YGNodeRef, w: f32, wm: YGMeasureMode, _h: f32, _hm: YGMeasureMode) -> YGSize {
    let measure = MEASURE_REF.as_mut().expect("measure not set");
    let key = *(YGNodeGetContext(node) as *mut usize);

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
