use crate::generated::{SurfaceId, Rect, UpdateSceneMsg, Size, Dimension, StyleProp, FlexAlign, JustifyContent, FlexDirection, FlexWrap, Dimensions};
use crate::layout::Layout;
use crate::SceneListener;
use stretch::geometry::{Size as StretchSize, Rect as StretchRect};
use stretch::Stretch;
use stretch::node::Node;
use stretch::style::{Style, Dimension as StretchDimension, AlignContent, AlignItems, AlignSelf, JustifyContent as StretchJustifyContent, FlexDirection as StretchFlexDirection, FlexWrap as StretchFlexWrap};
use stretch::number::Number;
use std::any::Any;

pub struct StretchLayout {
    stretch: Stretch,
    nodes: Vec<Node>,
    measure_text_holder: Option<&'static mut dyn FnMut(SurfaceId, Option<f32>) -> (f32, f32)>
}

impl StretchLayout {
    pub fn new((width, height): (f32, f32)) -> Self {
        let mut stretch = Stretch::new();

        let root = Self::new_node(&mut stretch);

        stretch.set_style(root, Style {
            size: StretchSize { width: StretchDimension::Points(width), height: StretchDimension::Points(height) },
            ..Default::default()
        });

        StretchLayout {
            stretch,
            nodes: vec![root],
            measure_text_holder: None
        }
    }

    fn new_node(stretch: &mut Stretch) -> Node {
        stretch.new_node(
            Style {
                flex_direction: StretchFlexDirection::Column,
                ..Default::default()
            },
            vec![]
        ).expect("couldn't create node")
    }

    fn update_style<F>(&mut self, surface: SurfaceId, mut update_fn: F) where F: FnMut(&mut Style) + Sized {
        let mut style = self.stretch.style(self.nodes[surface]).expect("no style").clone();

        update_fn(&mut style);

        self.stretch.set_style(self.nodes[surface], style);
    }
}

impl SceneListener for StretchLayout {
    fn update_scene(&mut self, msgs: &[UpdateSceneMsg]) {
        for m in msgs.iter().cloned() {
            match m {
                UpdateSceneMsg::Alloc => {
                    let node = StretchLayout::new_node(&mut self.stretch);
                    self.nodes.push(node);
                },
                // TODO: fork stretch & add insert_at()
                UpdateSceneMsg::InsertAt { parent, child, index } => {
                    let mut children = self.stretch.children(self.nodes[parent]).expect("couldnt get children");

                    children.insert(index, self.nodes[child]);

                    self.stretch.set_children(self.nodes[parent], children).expect("couldnt set children");
                },
                UpdateSceneMsg::RemoveChild { parent, child } => {
                    let parent = self.nodes[parent];
                    let child = self.nodes[child];

                    self.stretch.remove_child(parent, child).expect("couldnt remove");
                },
                UpdateSceneMsg::SetStyleProp { surface, prop } => {
                    match prop {
                        StyleProp::Size(s) => self.update_style(surface, |st| st.size = s.clone().into()),
                        StyleProp::Flex(f) => self.update_style(surface, |s| {
                            let f = f.clone();

                            s.flex_grow = f.flex_grow;
                            s.flex_shrink = f.flex_shrink;
                            s.flex_basis = f.flex_basis.into();
                        }),
                        StyleProp::Flow(f) => self.update_style(surface, |s| {
                            let f = f.clone();

                            s.flex_direction = f.flex_direction.into();
                            s.flex_wrap = f.flex_wrap.into();

                            s.align_items = f.align_items.into();
                            s.align_self = f.align_self.into();
                            s.align_content = f.align_content.into();
                            s.justify_content = f.justify_content.into();
                        }),
                        StyleProp::Padding(p) => self.update_style(surface, |s| s.padding = p.clone().into()),
                        StyleProp::Margin(m) => self.update_style(surface, |s| s.margin = m.clone().into()),
                        StyleProp::Text(t) => {
                            let node = self.nodes[surface];

                            if t.is_some() {
                                let stretch_layout = get_static_ref(self);

                                let measure_func: Box<dyn FnMut(StretchSize<Number>) -> Result<StretchSize<f32>, Box<dyn Any>>> = Box::new(move |size: StretchSize<Number>| {
                                    let max_width = match size.width {
                                        Number::Defined(w) => Some(w),
                                        Number::Undefined => None
                                    };

                                    let f = stretch_layout.measure_text_holder.as_mut().expect("not inside calculate");
                                    let res = f(surface, max_width);

                                    Ok(StretchSize { width: res.0, height: res.1 })
                                });

                                // it's FnMut but fuck it
                                self.stretch.set_measure(node, unsafe { std::mem::transmute(Some(measure_func)) });
                            } else {
                                self.stretch.set_measure(node, None);
                            }
                        },
                        /*
                        StyleProp::Border(b) => self.set_border(surface, b),
                        StyleProp::Overflow(o) => self.set_overflow(surface, o),
                        */
                        _ => {}
                    }
                }
            }
        }
    }
}

impl Layout for StretchLayout {
    fn calculate(&mut self, measure_text: &mut dyn FnMut(SurfaceId, Option<f32>) -> (f32, f32)) {
        self.measure_text_holder = Some(unsafe { std::mem::transmute(measure_text) });
        self.stretch.compute_layout(self.nodes[0], StretchSize::undefined()).expect("couldnt compute layout");
        self.measure_text_holder = None;
    }

    fn get_rect(&self, surface: SurfaceId) -> Rect {
        self.stretch.layout(self.nodes[surface]).expect("no layout").into()
    }

    fn get_scroll_frame(&self, surface: SurfaceId) -> Option<(f32, f32)> {
        None
    }
}

impl From<&stretch::result::Layout> for Rect {
    fn from(layout: &stretch::result::Layout) -> Rect {
        Rect(
            layout.location.x,
            layout.location.y,
            layout.size.width,
            layout.size.height,
        )
    }
}

impl Into<StretchSize<StretchDimension>> for Size {
    fn into(self) -> StretchSize<StretchDimension> {
        StretchSize {
            width: self.0.into(),
            height: self.1.into()
        }
    }
}

impl Into<StretchDimension> for Dimension {
    fn into(self) -> StretchDimension {
        match self {
            Dimension::Auto => StretchDimension::Auto,
            Dimension::Point(p) => StretchDimension::Points(p),
            Dimension::Percent(p) => StretchDimension::Percent(p),
        }
    }
}

impl Into<AlignItems> for FlexAlign {
    fn into(self) -> AlignItems {
        match self {
            FlexAlign::Baseline => AlignItems::Baseline,
            FlexAlign::Center => AlignItems::Center,
            FlexAlign::FlexStart => AlignItems::FlexStart,
            FlexAlign::FlexEnd => AlignItems::FlexEnd,
            FlexAlign::Stretch => AlignItems::Stretch,
            _ => unimplemented!()
        }
    }
}

impl Into<AlignSelf> for FlexAlign {
    fn into(self) -> AlignSelf {
        match self {
            FlexAlign::Auto => AlignSelf::Auto,
            FlexAlign::Baseline => AlignSelf::Baseline,
            FlexAlign::Center => AlignSelf::Center,
            FlexAlign::FlexStart => AlignSelf::FlexStart,
            FlexAlign::FlexEnd => AlignSelf::FlexEnd,
            FlexAlign::Stretch => AlignSelf::Stretch,
            _ => unimplemented!()
        }
    }
}

impl Into<AlignContent> for FlexAlign {
    fn into(self) -> AlignContent {
        match self {
            FlexAlign::Center => AlignContent::Center,
            FlexAlign::FlexStart => AlignContent::FlexStart,
            FlexAlign::FlexEnd => AlignContent::FlexEnd,
            FlexAlign::SpaceAround => AlignContent::SpaceAround,
            FlexAlign::SpaceBetween => AlignContent::SpaceBetween,
            FlexAlign::Stretch => AlignContent::Stretch,
            _ => unimplemented!()
        }
    }
}

impl Into<StretchJustifyContent> for JustifyContent {
    fn into(self) -> StretchJustifyContent {
        match self {
            JustifyContent::Center => StretchJustifyContent::Center,
            JustifyContent::FlexStart => StretchJustifyContent::FlexStart,
            JustifyContent::FlexEnd => StretchJustifyContent::FlexEnd,
            JustifyContent::SpaceAround => StretchJustifyContent::SpaceAround,
            JustifyContent::SpaceBetween => StretchJustifyContent::SpaceBetween,
            JustifyContent::SpaceEvenly => StretchJustifyContent::SpaceEvenly,
        }
    }
}

impl Into<StretchFlexDirection> for FlexDirection {
    fn into(self) -> StretchFlexDirection {
        match self {
            FlexDirection::Column => StretchFlexDirection::Column,
            FlexDirection::ColumnReverse => StretchFlexDirection::ColumnReverse,
            FlexDirection::Row => StretchFlexDirection::Row,
            FlexDirection::RowReverse => StretchFlexDirection::RowReverse,
        }
    }
}

impl Into<StretchFlexWrap> for FlexWrap {
    fn into(self) -> StretchFlexWrap {
        match self {
            FlexWrap::Wrap => StretchFlexWrap::Wrap,
            FlexWrap::WrapReverse => StretchFlexWrap::WrapReverse,
            FlexWrap::NoWrap => StretchFlexWrap::NoWrap,
        }
    }
}

impl Into<StretchRect<StretchDimension>> for Dimensions {
    fn into(self) -> StretchRect<StretchDimension> {
        StretchRect {
            top: self.0.into(),
            end: self.1.into(),
            bottom: self.2.into(),
            start: self.3.into()
        }
    }
}

pub fn get_static_ref(stretch_layout: &mut StretchLayout) -> &'static mut StretchLayout {
    unsafe { std::mem::transmute(stretch_layout) }
}
