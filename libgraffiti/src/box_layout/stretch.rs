// unmaintained for now

use crate::commons::{Pos, Bounds, SurfaceId, Border};
use crate::box_layout::{BoxLayout, DimensionProp, Dimension, AlignProp, Align, FlexDirection, FlexWrap};
use crate::text_layout::{Text};
use stretch::geometry::{Size as StretchSize};
use stretch::Stretch;
use stretch::node::Node;
use stretch::style::{Style as StretchStyle, Dimension as StretchDimension, AlignContent, AlignItems, AlignSelf, JustifyContent as StretchJustifyContent, FlexDirection as StretchFlexDirection, FlexWrap as StretchFlexWrap};
use stretch::number::{Number as StretchNumber};
use std::any::Any;

pub struct StretchLayout {
    window_size: StretchSize<StretchNumber>,
    stretch: Stretch,
    styles: Vec<StretchStyle>,
    nodes: Vec<Node>,
    bounds: Vec<Bounds>,
    measure_text_holder: Option<&'static mut dyn FnMut(SurfaceId, f32) -> (f32, f32)>
}

impl StretchLayout {
    pub fn new(width: i32, height: i32) -> Self {
        let mut res = StretchLayout {
            window_size: StretchSize::undefined(),
            stretch: Stretch::new(),
            nodes: Vec::new(),
            styles: Vec::new(),
            bounds: vec![Bounds::zero()],
            measure_text_holder: None
        };

        res.alloc();
        res.resize(width, height);

        res
    }

    fn update_style<F>(&mut self, surface: SurfaceId, mut update_fn: F) where F: FnMut(&mut StretchStyle) + Sized {
        let style = &mut self.styles[surface];

        update_fn(style);

        self.stretch.set_style(self.nodes[surface], *style).expect("update stretch style");
    }
}

impl BoxLayout for StretchLayout {
    fn alloc(&mut self) {
        let style = StretchStyle {
            flex_direction: StretchFlexDirection::Column,
            ..Default::default()
        };
        let node = self.stretch.new_node(style, vec![]).expect("couldn't create node");

        self.nodes.push(node);
        self.styles.push(style);
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

    fn set_dimension(&mut self, surface: SurfaceId, prop: DimensionProp, value: Dimension) {
        let v = value.into();

        self.update_style(surface, |s| {
            match prop {
                DimensionProp::Width => s.size.width = v,
                DimensionProp::Height => s.size.height = v,
                DimensionProp::MinWidth => s.min_size.width = v,
                DimensionProp::MinHeight => s.min_size.height = v,
                DimensionProp::MaxWidth => s.max_size.width = v,
                DimensionProp::MaxHeight => s.max_size.height = v,

                DimensionProp::PaddingLeft => s.padding.start = v,
                DimensionProp::PaddingRight => s.padding.end = v,
                DimensionProp::PaddingTop => s.padding.top = v,
                DimensionProp::PaddingBottom => s.padding.bottom = v,

                DimensionProp::MarginLeft => s.margin.start = v,
                DimensionProp::MarginRight => s.margin.end = v,
                DimensionProp::MarginTop => s.margin.top = v,
                DimensionProp::MarginBottom => s.margin.bottom = v,

                DimensionProp::FlexGrow => s.flex_grow = get_points(&v),
                DimensionProp::FlexShrink => s.flex_shrink = get_points(&v),
                DimensionProp::FlexBasis => s.flex_basis = v,
            }
        });
    }

    fn set_align(&mut self, surface: SurfaceId, prop: AlignProp, value: Align) {
        self.update_style(surface, |s| {
            match prop {
                AlignProp::AlignSelf => s.align_self = value.into(),
                AlignProp::AlignContent => s.align_content = value.into(),
                AlignProp::AlignItems => s.align_items = value.into(),
                AlignProp::JustifyContent => s.justify_content = value.into(),
            }
        })
    }

    fn set_flex_direction(&mut self, surface: SurfaceId, value: FlexDirection) {
        self.update_style(surface, |s| {
            s.flex_direction = value.into();
        });
    }

    fn set_flex_wrap(&mut self, surface: SurfaceId, value: FlexWrap) {
        self.update_style(surface, |s| {
            s.flex_wrap = value.into();
        });
    }

    fn set_border(&mut self, surface: SurfaceId, _border: Option<Border>) {
        self.update_style(surface, |_s| {
            error!("TODO: set border layout");
        });
    }

    fn set_text(&mut self, surface: SurfaceId, text: Option<Text>) {
        let node = self.nodes[surface];

        if text.is_some() {
            let stretch_layout = get_static_ref(self);

            let measure_func: Box<dyn FnMut(StretchSize<StretchNumber>) -> Result<StretchSize<f32>, Box<dyn Any>>> = Box::new(move |size: StretchSize<StretchNumber>| {
                let max_width = match size.width {
                    StretchNumber::Defined(w) => w,
                    StretchNumber::Undefined => std::f32::MAX
                };

                let f = stretch_layout.measure_text_holder.as_mut().expect("not inside calculate");
                let (width, height) = f(surface, max_width);

                Ok(StretchSize { width, height })
            });

            // it's FnMut but fuck it
            self.stretch.set_measure(node, unsafe { std::mem::transmute(Some(measure_func)) }).expect("set measure");
        } else {
            self.stretch.set_measure(node, None).expect("set measure");
        }
    }

    fn resize(&mut self, width: i32, height: i32) {
        self.window_size = StretchSize {
            width: StretchNumber::Defined(width as f32),
            height: StretchNumber::Defined(height as f32)
        };

        self.update_style(0, |s| {
            s.size = StretchSize {
                width: StretchDimension::Points(width as f32),
                height: StretchDimension::Points(height as f32)
            };
        });
    }

    // TODO: stretch for some reason calls measure quite often
    fn calculate(&mut self, measure_text: &mut dyn FnMut(SurfaceId, f32) -> (f32, f32)) {
        self.measure_text_holder = Some(unsafe { std::mem::transmute(measure_text) });
        self.stretch.compute_layout(self.nodes[0], self.window_size).expect("couldnt compute layout");
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

// hacky because of into(), type inference & DRY :-/
fn get_points(dim: &StretchDimension) -> f32 {
    match dim {
        StretchDimension::Points(v) => *v,
        _ => panic!("expected point")
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

impl Into<AlignItems> for Align {
    fn into(self) -> AlignItems {
        match self {
            Align::Baseline => AlignItems::Baseline,
            Align::Center => AlignItems::Center,
            Align::FlexStart => AlignItems::FlexStart,
            Align::FlexEnd => AlignItems::FlexEnd,
            Align::Stretch => AlignItems::Stretch,
            _ => unimplemented!()
        }
    }
}

impl Into<AlignSelf> for Align {
    fn into(self) -> AlignSelf {
        match self {
            Align::Auto => AlignSelf::Auto,
            Align::Baseline => AlignSelf::Baseline,
            Align::Center => AlignSelf::Center,
            Align::FlexStart => AlignSelf::FlexStart,
            Align::FlexEnd => AlignSelf::FlexEnd,
            Align::Stretch => AlignSelf::Stretch,
            _ => unimplemented!()
        }
    }
}

impl Into<AlignContent> for Align {
    fn into(self) -> AlignContent {
        match self {
            Align::Center => AlignContent::Center,
            Align::FlexStart => AlignContent::FlexStart,
            Align::FlexEnd => AlignContent::FlexEnd,
            Align::SpaceAround => AlignContent::SpaceAround,
            Align::SpaceBetween => AlignContent::SpaceBetween,
            Align::Stretch => AlignContent::Stretch,
            _ => unimplemented!()
        }
    }
}

impl Into<StretchJustifyContent> for Align {
    fn into(self) -> StretchJustifyContent {
        match self {
            Align::Center => StretchJustifyContent::Center,
            Align::FlexStart => StretchJustifyContent::FlexStart,
            Align::FlexEnd => StretchJustifyContent::FlexEnd,
            Align::SpaceAround => StretchJustifyContent::SpaceAround,
            Align::SpaceBetween => StretchJustifyContent::SpaceBetween,
            Align::SpaceEvenly => StretchJustifyContent::SpaceEvenly,
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

pub fn get_static_ref(stretch_layout: &mut StretchLayout) -> &'static mut StretchLayout {
    unsafe { std::mem::transmute(stretch_layout) }
}
