// TODO: generic MeasureKey
// (should be possible, at least we can save pointer in the context
//  and the closure (dyn Fn) is generic so it should know the size of the key)

use super::{Align, Dimension, Display, FlexDirection, FlexWrap, Justify, LayoutEngine, LayoutStyle, Overflow};
use graffiti_yoga::*;

pub struct YogaLayoutEngine {
    yoga_nodes: Vec<YGNodeRef>,
}

// should be safe (no thread-locals, etc.)
unsafe impl std::marker::Send for YogaLayoutEngine {}

impl YogaLayoutEngine {
    pub fn new() -> Self {
        YogaLayoutEngine { yoga_nodes: Vec::new() }
    }
}

// call respective YG* functions depending on the dimension type
// $set_auto is optional
macro_rules! set_dim {
    ($node:expr, $value:expr, $set:ident $set_perc:ident $($set_auto:ident)*) => (
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
    ($node:expr, $value:expr, $edge:ident $set:ident $set_perc:ident $($set_auto:ident)*) => (
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

impl LayoutEngine for YogaLayoutEngine {
    type LayoutNodeId = YGNodeRef;
    type MeasureKey = usize;

    fn create_node(&mut self, style: &LayoutStyle) -> YGNodeRef {
        let node = unsafe { YGNodeNew() };

        self.set_style(node, style);
        self.yoga_nodes.push(node);

        node
    }

    fn set_style(&mut self, node: Self::LayoutNodeId, style: &LayoutStyle) {
        let s = style;

        unsafe {
            YGNodeStyleSetOverflow(node, s.overflow.into());

            YGNodeStyleSetFlexGrow(node, s.flex.grow);
            YGNodeStyleSetFlexShrink(node, s.flex.shrink);
            set_dim!(node, s.flex.basis, YGNodeStyleSetFlexBasis YGNodeStyleSetFlexBasisPercent YGNodeStyleSetFlexBasisAuto);

            set_dim!(node, s.width, YGNodeStyleSetWidth YGNodeStyleSetWidthPercent YGNodeStyleSetWidthAuto);
            set_dim!(node, s.height, YGNodeStyleSetHeight YGNodeStyleSetHeightPercent YGNodeStyleSetHeightAuto);
            set_dim!(node, s.min_width, YGNodeStyleSetMinWidth YGNodeStyleSetMinWidthPercent);
            set_dim!(node, s.min_height, YGNodeStyleSetMinHeight YGNodeStyleSetMinHeightPercent);
            set_dim!(node, s.max_width, YGNodeStyleSetMaxWidth YGNodeStyleSetMaxWidthPercent);
            set_dim!(node, s.max_height, YGNodeStyleSetMaxHeight YGNodeStyleSetMaxHeightPercent);

            set_dim_edge!(node, s.margin.top, Top YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto);
            set_dim_edge!(node, s.margin.right, Right YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto);
            set_dim_edge!(node, s.margin.bottom, Bottom YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto);
            set_dim_edge!(node, s.margin.left, Left YGNodeStyleSetMargin YGNodeStyleSetMarginPercent YGNodeStyleSetMarginAuto);

            set_dim_edge!(node, s.padding.top, Top YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent);
            set_dim_edge!(node, s.padding.right, Right YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent);
            set_dim_edge!(node, s.padding.bottom, Bottom YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent);
            set_dim_edge!(node, s.padding.left, Left YGNodeStyleSetPadding YGNodeStyleSetPaddingPercent);

            YGNodeStyleSetBorder(node, YGEdge::Top, s.border.top);
            YGNodeStyleSetBorder(node, YGEdge::Right, s.border.right);
            YGNodeStyleSetBorder(node, YGEdge::Bottom, s.border.bottom);
            YGNodeStyleSetBorder(node, YGEdge::Left, s.border.left);

            YGNodeStyleSetFlexDirection(node, s.flex_flow.direction.into());
            YGNodeStyleSetFlexWrap(node, s.flex_flow.wrap.into());
            YGNodeStyleSetAlignItems(node, s.align_items.into());
            YGNodeStyleSetAlignContent(node, s.align_content.into());
            YGNodeStyleSetJustifyContent(node, s.justify_content.into());
            YGNodeStyleSetAlignSelf(node, s.align_self.into());

            set_dim_edge!(node, s.top, Top YGNodeStyleSetPosition YGNodeStyleSetPositionPercent);
            set_dim_edge!(node, s.right, Right YGNodeStyleSetPosition YGNodeStyleSetPositionPercent);
            set_dim_edge!(node, s.bottom, Bottom YGNodeStyleSetPosition YGNodeStyleSetPositionPercent);
            set_dim_edge!(node, s.left, Left YGNodeStyleSetPosition YGNodeStyleSetPositionPercent);

            match s.display {
                Display::Block => {
                    // emulate display: block using flexbox
                    //
                    // width: 100% wouldn't work when inside flex (<Row>) because
                    // it would take the whole row and push the rest to the side
                    // (but align-items: stretch should be enough in most cases)
                    //self.set_width(element, Dimension::Percent { value: 100. });

                    YGNodeStyleSetDisplay(node, YGDisplay::Flex);
                    YGNodeStyleSetFlexDirection(node, FlexDirection::Column.into());
                    YGNodeStyleSetAlignItems(node, Align::Stretch.into());
                }

                Display::Flex => YGNodeStyleSetDisplay(node, YGDisplay::Flex),
                Display::None => YGNodeStyleSetDisplay(node, YGDisplay::None),
            }
        }
    }

    fn insert_child(&mut self, parent: YGNodeRef, index: usize, child: YGNodeRef) {
        unsafe { YGNodeInsertChild(parent, child, index as u32) }
    }

    fn remove_child(&mut self, parent: YGNodeRef, child: YGNodeRef) {
        unsafe { YGNodeRemoveChild(parent, child) }
    }

    fn create_leaf(&mut self, measure_key: Self::MeasureKey) -> Self::LayoutNodeId {
        let node = unsafe { YGNodeNew() };

        unsafe {
            YGNodeSetMeasureFunc(node, Some(measure_text_node));
            YGNodeMarkDirty(node);
            YGNodeSetContext(node, std::boxed::Box::<Self::MeasureKey>::leak(Box::new(measure_key)) as *mut usize as *mut std::ffi::c_void);
        }

        self.yoga_nodes.push(node);

        node
    }

    fn mark_dirty(&mut self, node: YGNodeRef) {
        unsafe { YGNodeMarkDirty(node) }
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

    #[inline]
    fn get_offset(&self, node: YGNodeRef) -> (f32, f32) {
        unsafe { (YGNodeLayoutGetLeft(node), YGNodeLayoutGetTop(node)) }
    }

    #[inline]
    fn get_size(&self, node: YGNodeRef) -> (f32, f32) {
        unsafe { (YGNodeLayoutGetWidth(node), YGNodeLayoutGetHeight(node)) }
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
        }
    }
}

impl Into<YGJustify> for Justify {
    fn into(self) -> YGJustify {
        match self {
            Justify::Center => YGJustify::Center,
            Justify::FlexStart => YGJustify::FlexStart,
            Justify::FlexEnd => YGJustify::FlexEnd,
            Justify::SpaceAround => YGJustify::SpaceAround,
            Justify::SpaceBetween => YGJustify::SpaceBetween,
            Justify::SpaceEvenly => YGJustify::SpaceEvenly,
        }
    }
}

impl Into<YGFlexDirection> for FlexDirection {
    fn into(self) -> YGFlexDirection {
        match self {
            FlexDirection::Row => YGFlexDirection::Row,
            FlexDirection::RowReverse => YGFlexDirection::RowReverse,
            FlexDirection::Column => YGFlexDirection::Column,
            FlexDirection::ColumnReverse => YGFlexDirection::ColumnReverse,
        }
    }
}

impl Into<YGWrap> for FlexWrap {
    fn into(self) -> YGWrap {
        match self {
            FlexWrap::NoWrap => YGWrap::NoWrap,
            FlexWrap::Wrap => YGWrap::Wrap,
            FlexWrap::WrapReverse => YGWrap::WrapReverse,
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
