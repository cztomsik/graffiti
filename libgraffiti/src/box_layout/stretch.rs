use crate::commons::{Pos, Bounds, SurfaceId, Border};
use crate::box_layout::{BoxLayout, Layout, Dimension, FlexAlign, FlexDirection, FlexWrap, Dimensions};
use crate::text_layout::{Text};
use stretch::geometry::{Size as StretchSize, Rect as StretchRect};
use stretch::Stretch;
use stretch::node::Node;
use stretch::style::{Style, Dimension as StretchDimension, AlignContent, AlignItems, AlignSelf, JustifyContent as StretchJustifyContent, FlexDirection as StretchFlexDirection, FlexWrap as StretchFlexWrap};
use stretch::number::Number;
use std::any::Any;

pub struct StretchLayout {
    stretch: Stretch,
    nodes: Vec<Node>,
    bounds: Vec<Bounds>,
    measure_text_holder: Option<&'static mut dyn FnMut(SurfaceId, Option<f32>) -> (f32, f32)>
}

impl StretchLayout {
    pub fn new((width, height): (f32, f32)) -> Self {
        let mut stretch = Stretch::new();

        let root = Self::new_node(&mut stretch);

        stretch.set_style(root, Style {
            size: StretchSize { width: StretchDimension::Points(width), height: StretchDimension::Points(height) },
            ..Default::default()
        }).expect("init root layout");

        StretchLayout {
            stretch,
            nodes: vec![root],
            bounds: vec![Bounds::zero()],
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
        // TODO: hopefully it's not too costy, otherwise we could cache it
        let mut style = self.stretch.style(self.nodes[surface]).expect("no style").clone();

        update_fn(&mut style);

        self.stretch.set_style(self.nodes[surface], style).expect("update layout");
    }
}

impl BoxLayout for StretchLayout {
    fn alloc(&mut self) {
        let node = StretchLayout::new_node(&mut self.stretch);
        self.nodes.push(node);
        self.bounds.push(Bounds::zero());
    }

    // TODO: fork stretch & add insert_at()
    fn insert_at(&mut self, parent: SurfaceId, child: SurfaceId, index: usize) {
        let mut children = self.stretch.children(self.nodes[parent]).expect("couldnt get children");

        children.insert(index, self.nodes[child]);

        self.stretch.set_children(self.nodes[parent], children).expect("couldnt set children");
    }

    fn remove_child(&mut self, parent: SurfaceId, child: SurfaceId) {
        let parent = self.nodes[parent];
        let child = self.nodes[child];

        self.stretch.remove_child(parent, child).expect("couldnt remove");
    }

    fn set_layout(&mut self, surface: SurfaceId, layout: Layout) {
        self.update_style(surface, |s| {
            let layout = layout.clone();

            s.size = StretchSize { width: layout.width.into(), height: layout.height.into() };

            s.flex_grow = layout.flex_grow;
            s.flex_shrink = layout.flex_shrink;
            s.flex_basis = layout.flex_basis.into();
            s.flex_direction = layout.flex_direction.into();
            s.flex_wrap = layout.flex_wrap.into();

            s.align_items = layout.align_items.into();
            s.align_self = layout.align_self.into();
            s.align_content = layout.align_content.into();
            s.justify_content = layout.justify_content.into();

            s.margin = layout.margin.into();
            s.padding = layout.padding.into();

        })
    }

    fn set_border(&mut self, surface: SurfaceId, _border: Option<Border>) {
        self.update_style(surface, |_s| {
            debug!("TODO: set border layout");
        })
    }

    fn set_text(&mut self, surface: SurfaceId, text: Option<Text>) {
        let node = self.nodes[surface];

        if text.is_some() {
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
            self.stretch.set_measure(node, unsafe { std::mem::transmute(Some(measure_func)) }).expect("set measure");
        } else {
            self.stretch.set_measure(node, None).expect("set measure");
        }
    }

    fn calculate(&mut self, measure_text: &mut dyn FnMut(SurfaceId, Option<f32>) -> (f32, f32)) {
        self.measure_text_holder = Some(unsafe { std::mem::transmute(measure_text) });
        self.stretch.compute_layout(self.nodes[0], StretchSize::undefined()).expect("couldnt compute layout");
        self.measure_text_holder = None;

        // TODO: update only attached and display != none nodes
        for i in 0..self.nodes.len() {
            self.bounds[i] = self.stretch.layout(self.nodes[i]).expect("no layout").into()
        }
    }

    fn get_bounds(&self) -> &[Bounds] {
        &self.bounds
    }
}

impl From<&stretch::result::Layout> for Bounds {
    fn from(layout: &stretch::result::Layout) -> Bounds {
        let a = Pos::new(layout.location.x, layout.location.y);
        let b = Pos::new(layout.location.x + layout.size.width, layout.location.y + layout.size.height);

        Bounds { a, b }
    }
}

impl Into<StretchDimension> for Dimension {
    fn into(self) -> StretchDimension {
        match self {
            Dimension { point: None, percent: None } => StretchDimension::Auto,
            Dimension { point: Some(p), .. } => StretchDimension::Points(p),
            Dimension { percent: Some(p), .. } => StretchDimension::Percent(p),
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

impl Into<StretchJustifyContent> for FlexAlign {
    fn into(self) -> StretchJustifyContent {
        match self {
            FlexAlign::Center => StretchJustifyContent::Center,
            FlexAlign::FlexStart => StretchJustifyContent::FlexStart,
            FlexAlign::FlexEnd => StretchJustifyContent::FlexEnd,
            FlexAlign::SpaceAround => StretchJustifyContent::SpaceAround,
            FlexAlign::SpaceBetween => StretchJustifyContent::SpaceBetween,
            FlexAlign::SpaceEvenly => StretchJustifyContent::SpaceEvenly,
            _ => unimplemented!()            
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
            top: self.top.into(),
            end: self.right.into(),
            bottom: self.bottom.into(),
            start: self.left.into()
        }
    }
}

pub fn get_static_ref(stretch_layout: &mut StretchLayout) -> &'static mut StretchLayout {
    unsafe { std::mem::transmute(stretch_layout) }
}
