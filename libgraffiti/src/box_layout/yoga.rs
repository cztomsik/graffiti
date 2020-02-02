use crate::commons::{ElementId, TextId, ElementChild, Pos, Bounds};
use super::{BoxLayoutTree, Display, Dimension, Align, FlexDirection, FlexWrap, Overflow};
use graffiti_yoga::*;

pub struct YogaLayoutTree {
    element_yoga_nodes: Vec<YGNodeRef>,
    text_yoga_nodes: Vec<YGNodeRef>,
}

// should be safe (no thread-locals, etc.)
unsafe impl std::marker::Send for YogaLayoutTree {}

impl YogaLayoutTree {
    pub fn new() -> Self {
        YogaLayoutTree {
            element_yoga_nodes: Vec::new(),
            text_yoga_nodes: Vec::new(),
        }
    }
}

// generate setters
// unfortunately, macros can't prefix idents
// $set_auto is optional
macro_rules! dim_setter {
    ($meth:ident $set:ident $set_perc:ident $($set_auto:ident)*) => (
        #[allow(unreachable_patterns)]
        fn $meth(&mut self, element: ElementId, v: Dimension) {
            let node = self.element_yoga_nodes[element];

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
        fn $meth(&mut self, element: ElementId, v: Dimension) {
            let node = self.element_yoga_nodes[element];

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

impl BoxLayoutTree for YogaLayoutTree {
    fn realloc(&mut self, elements_count: ElementId, texts_count: TextId) {
        let new_element_ids = self.element_yoga_nodes.len()..elements_count;
        let new_text_ids = self.text_yoga_nodes.len()..texts_count;

        self.element_yoga_nodes.resize_with(elements_count, || unsafe { YGNodeNew() });
        self.text_yoga_nodes.resize_with(texts_count, || unsafe { YGNodeNew() });

        for id in new_element_ids {
            // TODO: in the browser, default display is "inline", we dont support that
            // but this is wrong too (flex)

            // set web defaults
            self.set_flex_direction(id, FlexDirection::Row);
            self.set_flex_basis(id, Dimension::Auto);
            self.set_flex_shrink(id, 1.);
        }

        for id in new_text_ids {
            unsafe {
                let node = self.text_yoga_nodes[id];

                YGNodeSetMeasureFunc(node, Some(measure_text_node));
                YGNodeMarkDirty(node);
                YGNodeSetContext(node, std::boxed::Box::<TextId>::leak(Box::new(id)) as *mut usize as *mut std::ffi::c_void);
            }
        }
    }

    fn insert_at(&mut self, parent: ElementId, child: ElementChild, index: usize) {
        let parent = self.element_yoga_nodes[parent];
        let index = index as u32;

        unsafe {
            match child {
                ElementChild::Element { id } => YGNodeInsertChild(parent, self.element_yoga_nodes[id], index),
                ElementChild::Text { id } => YGNodeInsertChild(parent, self.text_yoga_nodes[id], index),
            }
        }
    }

    fn remove_child(&mut self, parent: ElementId, child: ElementChild) {
        let parent = self.element_yoga_nodes[parent];

        unsafe {
            match child {
                ElementChild::Element { id } => YGNodeRemoveChild(parent, self.element_yoga_nodes[id]),
                ElementChild::Text { id } => YGNodeRemoveChild(parent, self.text_yoga_nodes[id]),
            }
        }
    }

    fn calculate(&mut self, element: ElementId, (width, _height): (f32, f32), measure_fn: &mut dyn FnMut(TextId, f32) -> (f32, f32)) {
        unsafe {
            if MEASURE_REF.is_some() {
                panic!("layout not thread-safe");
            }

            MEASURE_REF = Some(std::mem::transmute(measure_fn));

            // height is ignored (yoga treats it as maxHeight which is not what we want)
            // @see resize() in viewport.rs
            YGNodeCalculateLayout(self.element_yoga_nodes[element], width, YGUndefined, YGDirection::LTR);

            MEASURE_REF = None;
        }
    }

    #[inline(always)]
    fn get_element_bounds(&self, element: ElementId) -> Bounds {
        get_yoga_bounds(self.element_yoga_nodes[element])
    }

    #[inline(always)]
    fn get_text_bounds(&self, text: TextId) -> Bounds {
        get_yoga_bounds(self.text_yoga_nodes[text])
    }

    fn set_display(&mut self, element: ElementId, v: Display) {
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
                self.set_flex_direction(element, FlexDirection::Column);
                self.set_align_items(element, Align::Stretch);
            },
            Display::Flex => {
                self.set_flex_direction(element, FlexDirection::Row);
            },
        }
    }

    fn set_overflow(&mut self, element: ElementId, v: Overflow) {
        unsafe {
            YGNodeStyleSetOverflow(self.element_yoga_nodes[element], match v {
                Overflow::Visible => YGOverflow::Visible,
                Overflow::Hidden => YGOverflow::Hidden,
                Overflow::Scroll => YGOverflow::Scroll,
            })
        }
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

    fn set_border_top(&mut self, element: ElementId, v: f32) {
        unsafe { YGNodeStyleSetBorder(self.element_yoga_nodes[element], YGEdge::Top, v) }
    }

    fn set_border_right(&mut self, element: ElementId, v: f32) {
        unsafe { YGNodeStyleSetBorder(self.element_yoga_nodes[element], YGEdge::Right, v) }
    }

    fn set_border_bottom(&mut self, element: ElementId, v: f32) {
        unsafe { YGNodeStyleSetBorder(self.element_yoga_nodes[element], YGEdge::Bottom, v) }
    }

    fn set_border_left(&mut self, element: ElementId, v: f32) {
        unsafe { YGNodeStyleSetBorder(self.element_yoga_nodes[element], YGEdge::Left, v) }
    }

    fn set_flex_grow(&mut self, element: ElementId, v: f32) {
        unsafe { YGNodeStyleSetFlexGrow(self.element_yoga_nodes[element], v) }
    }

    fn set_flex_shrink(&mut self, element: ElementId, v: f32) {
        unsafe { YGNodeStyleSetFlexShrink(self.element_yoga_nodes[element], v) }
    }

    dim_setter!(set_flex_basis YGNodeStyleSetFlexBasis YGNodeStyleSetFlexBasisPercent YGNodeStyleSetFlexBasisAuto);

    fn set_flex_direction(&mut self, element: ElementId, v: FlexDirection) {
        unsafe { YGNodeStyleSetFlexDirection(self.element_yoga_nodes[element], v.into()) }
    }

    fn set_flex_wrap(&mut self, element: ElementId, v: FlexWrap) {
        unsafe { YGNodeStyleSetFlexWrap(self.element_yoga_nodes[element], v.into()) }
    }

    fn set_align_self(&mut self, element: ElementId, v: Align) {
        unsafe { YGNodeStyleSetAlignSelf(self.element_yoga_nodes[element], v.into()) }
    }

    fn set_align_content(&mut self, element: ElementId, v: Align) {
        unsafe { YGNodeStyleSetAlignContent(self.element_yoga_nodes[element], v.into()) }
    }

    fn set_align_items(&mut self, element: ElementId, v: Align) {
        unsafe { YGNodeStyleSetAlignItems(self.element_yoga_nodes[element], v.into()) }
    }

    fn set_justify_content(&mut self, element: ElementId, v: Align) {
        unsafe { YGNodeStyleSetJustifyContent(self.element_yoga_nodes[element], v.into()) }
    }

    fn mark_text_dirty(&mut self, text: TextId) {
        unsafe { YGNodeMarkDirty(self.text_yoga_nodes[text]) }
    }
}

static mut MEASURE_REF: Option<&'static mut dyn FnMut(TextId, f32) -> (f32, f32)> = None;

#[inline(always)]
fn get_yoga_bounds(node: YGNodeRef) -> Bounds {
    unsafe {
        let left = YGNodeLayoutGetLeft(node);
        let top = YGNodeLayoutGetTop(node);

        let a = Pos::new(left, top);
        let b = Pos::new(left + YGNodeLayoutGetWidth(node), top + YGNodeLayoutGetHeight(node));

        Bounds { a, b }
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
    let measure = MEASURE_REF.as_mut().expect("measure not set");
    let key = *(YGNodeGetContext(node) as *mut TextId);

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
