// x owns the systems
// x connect/orchestrate them together

use crate::box_layout::{Align, BoxLayoutImpl, BoxLayoutTree, Dimension, Display, FlexDirection, FlexWrap, Overflow};
use crate::commons::{Bounds, Pos};
use crate::render::backend::gl::GlRenderBackend;
use crate::render::backend::RenderBackend;
use crate::render::value_types::{BorderStyle, Color};
use crate::render::{Renderer, SurfaceId};
use crate::text_layout::{Text, TextId, TextLayout};
use std::collections::BTreeMap;

pub type GlViewport = Viewport<GlRenderBackend>;

pub struct Viewport<RB: RenderBackend> {
    size: (f32, f32),

    // systems
    box_layout: BoxLayoutImpl,
    text_layout: TextLayout,
    renderer: Renderer<RB, <BoxLayoutImpl as BoxLayoutTree>::LayoutNodeId>,

    // handles
    layout_nodes: Vec<<BoxLayoutImpl as BoxLayoutTree>::LayoutNodeId>,
    surfaces: Vec<SurfaceId>,
    texts: BTreeMap<NodeId, (TextId, f32)>,

    // flags
    needs_layout: bool,

    // TODO: fullscreen + track the element id
    // (and use it in render/calculate)
    mouse_pos: Pos,
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    MouseMove { target: NodeId },
    MouseDown { target: NodeId },
    MouseUp { target: NodeId },
    Scroll { target: NodeId },
    KeyDown { target: NodeId, key: u16 },
    KeyPress { target: NodeId, key: u16 },
    KeyUp { target: NodeId, key: u16 },
    Resize { target: NodeId },
    Close { target: NodeId },
}

// supported style props
#[derive(Debug, Clone, Copy)]
pub enum StyleProp {
    Display(Display),
    Overflow(Overflow),

    Width(Dimension),
    Height(Dimension),
    MinWidth(Dimension),
    MinHeight(Dimension),
    MaxWidth(Dimension),
    MaxHeight(Dimension),

    Top(Dimension),
    Right(Dimension),
    Bottom(Dimension),
    Left(Dimension),

    MarginTop(Dimension),
    MarginRight(Dimension),
    MarginBottom(Dimension),
    MarginLeft(Dimension),

    PaddingTop(Dimension),
    PaddingRight(Dimension),
    PaddingBottom(Dimension),
    PaddingLeft(Dimension),

    FlexGrow(f32),
    FlexShrink(f32),
    FlexBasis(Dimension),
    FlexDirection(FlexDirection),
    FlexWrap(FlexWrap),

    AlignSelf(Align),
    AlignContent(Align),
    AlignItems(Align),
    JustifyContent(Align),

    Color(Color),
    BackgroundColor(Color),

    // TODO: border color
    BorderTopLeftRadius(f32),
    BorderTopRightRadius(f32),
    BorderBottomLeftRadius(f32),
    BorderBottomRightRadius(f32),
    BorderTopWidth(f32),
    BorderRightWidth(f32),
    BorderBottomWidth(f32),
    BorderLeftWidth(f32),
    BorderTopStyle(BorderStyle),
    BorderRightStyle(BorderStyle),
    BorderBottomStyle(BorderStyle),
    BorderLeftStyle(BorderStyle),
    // BackgroundImageUrl(String),

    // TODO: split to outline_shadows & inset_shadows
    //BoxShadow(Vec<BoxShadow>),

    // TODO: many
    //Transform(Vec<Transform>),
}

impl<RB: RenderBackend> Viewport<RB> {
    pub const ROOT: NodeId = 0;

    pub fn new(backend: RB, size: (f32, f32)) -> Viewport<RB> {
        let mut viewport = Viewport {
            size,

            box_layout: BoxLayoutImpl::new(),
            text_layout: TextLayout::new(),
            renderer: Renderer::new(backend),

            needs_layout: false,

            layout_nodes: Vec::new(),
            surfaces: Vec::new(),
            texts: BTreeMap::new(),

            mouse_pos: Pos::ZERO,
        };

        // create root
        viewport.create_element();

        // set min-height
        viewport.resize(size);

        viewport
    }

    pub fn create_element(&mut self) -> NodeId {
        let id = self.layout_nodes.len();
        let layout_node = self.box_layout.create_node(None);

        self.layout_nodes.push(layout_node);
        self.surfaces.push(self.renderer.create_surface(layout_node));

        id
    }

    pub fn create_text_node(&mut self) -> NodeId {
        let id = self.layout_nodes.len();
        let text = self.text_layout.create_text();
        let layout_node = self.box_layout.create_node(Some(text));

        self.texts.insert(id, (text, 0.));
        self.layout_nodes.push(layout_node);
        self.surfaces.push(self.renderer.create_surface(layout_node));

        id
    }

    pub fn insert_child(&mut self, parent: NodeId, index: usize, child: NodeId) {
        self.box_layout.insert_child(self.layout_nodes[parent], index, self.layout_nodes[child]);
        self.renderer.insert_child(self.surfaces[parent], index, self.surfaces[child]);
    }

    pub fn remove_child(&mut self, parent: NodeId, child: NodeId) {
        self.box_layout.remove_child(self.layout_nodes[parent], self.layout_nodes[child]);
        self.renderer.remove_child(self.surfaces[parent], self.surfaces[child]);
    }

    pub fn set_text(&mut self, id: NodeId, data: &Text) {
        let (text, prev_width) = self.texts.get_mut(&id).expect("not a text node");

        self.box_layout.mark_dirty(self.layout_nodes[id]);
        self.text_layout.set_text(*text, &data);

        // set to 0. so the glyphs are invalidated and uploaded during `update()`
        *prev_width = 0.;
    }

    pub fn set_style(&mut self, element: NodeId, prop: &StyleProp) {
        use StyleProp::*;

        let surface = self.surfaces[element];

        // order by likelihood (perf)
        match prop {
            // start with layout-independent things

            //Transform(v) => self.renderer.set_transform(surface, *v),
            Color(v) => self.renderer.set_color(surface, *v),
            BackgroundColor(v) => self.renderer.set_background_color(surface, *v),

            //BoxShadow(v) => self.renderer.set_box_shadow(surface, *v),

            // TODO: BorderRadius(v) => self.renderer.set_border_radius(surface, *v),

            // TODO: Image

            // TODO: Border
            // might need relayout!

            // layout will be needed
            c => {
                self.needs_layout = true;

                let layout_node = self.layout_nodes[element];

                match c {
                    Display(v) => self.box_layout.set_display(layout_node, *v),

                    Width(v) => self.box_layout.set_width(layout_node, *v),
                    Height(v) => self.box_layout.set_height(layout_node, *v),
                    MinWidth(v) => self.box_layout.set_min_width(layout_node, *v),
                    MinHeight(v) => self.box_layout.set_min_height(layout_node, *v),
                    MaxWidth(v) => self.box_layout.set_max_width(layout_node, *v),
                    MaxHeight(v) => self.box_layout.set_max_height(layout_node, *v),

                    Top(v) => self.box_layout.set_top(layout_node, *v),
                    Right(v) => self.box_layout.set_right(layout_node, *v),
                    Bottom(v) => self.box_layout.set_bottom(layout_node, *v),
                    Left(v) => self.box_layout.set_left(layout_node, *v),

                    MarginTop(v) => self.box_layout.set_margin_top(layout_node, *v),
                    MarginRight(v) => self.box_layout.set_margin_right(layout_node, *v),
                    MarginBottom(v) => self.box_layout.set_margin_bottom(layout_node, *v),
                    MarginLeft(v) => self.box_layout.set_margin_left(layout_node, *v),

                    PaddingTop(v) => self.box_layout.set_padding_top(layout_node, *v),
                    PaddingRight(v) => self.box_layout.set_padding_right(layout_node, *v),
                    PaddingBottom(v) => self.box_layout.set_padding_bottom(layout_node, *v),
                    PaddingLeft(v) => self.box_layout.set_padding_left(layout_node, *v),

                    FlexGrow(v) => self.box_layout.set_flex_grow(layout_node, *v),
                    FlexShrink(v) => self.box_layout.set_flex_shrink(layout_node, *v),
                    FlexBasis(v) => self.box_layout.set_flex_basis(layout_node, *v),
                    FlexDirection(v) => self.box_layout.set_flex_direction(layout_node, *v),
                    FlexWrap(v) => self.box_layout.set_flex_wrap(layout_node, *v),

                    AlignSelf(v) => self.box_layout.set_align_self(layout_node, *v),
                    AlignContent(v) => self.box_layout.set_align_content(layout_node, *v),
                    AlignItems(v) => self.box_layout.set_align_items(layout_node, *v),
                    JustifyContent(v) => self.box_layout.set_justify_content(layout_node, *v),

                    BorderBottomWidth(v) => self.box_layout.set_border_bottom(layout_node, *v),
                    BorderLeftWidth(v) => self.box_layout.set_border_left(layout_node, *v),
                    BorderRightWidth(v) => self.box_layout.set_border_right(layout_node, *v),
                    BorderTopWidth(v) => self.box_layout.set_border_top(layout_node, *v),

                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn update(&mut self) {
        silly!("update/render");

        let Self { box_layout, text_layout, .. } = self;

        if self.needs_layout {
            // TODO: replace with Lookup<(id, w), (f32, f32)> (blocked by immutable measure())
            box_layout.calculate(self.layout_nodes[Self::ROOT], self.size, &mut |text_id, max_width| {
                text_layout.wrap(text_id, max_width)
            });

            self.needs_layout = false;
        }

        // TODO: go through texts and find if it's needed to upload new glyphs
        // this sounds naive but it's simple, it should be at least same efficient as pull-from-cache-during-render
        // approach because it's more cache-friendlier
        //
        // and what's most important, it should be possible to run it in parallel
        // (work-stealing and/or concurent with the rest of render preparation)

        // TODO: split preparation and rendering so it's more thread-friendly
        self.renderer.render_surface(self.surfaces[Self::ROOT], &|k| box_layout.get_bounds(k));
    }

    pub fn get_offset_bounds(&self, element: NodeId) -> Bounds {
        self.box_layout.get_bounds(self.layout_nodes[element])
    }

    fn get_mouse_target(&self) -> NodeId {
        Self::ROOT
        //self.picker.pick_at(self.mouse_pos, &self.children, &self.box_layout)
    }

    // translate events (break coupling)
    // app delegates to platform, which gets native events & calls
    // these methods to get events specific to this window/viewport
    // apart from the method signature, there's no need for other abstractions

    pub fn mouse_move(&mut self, pos: Pos) -> Event {
        self.mouse_pos = pos;

        Event::MouseMove { target: self.get_mouse_target() }
    }

    /*
    pub fn scroll(&mut self, _delta: (f32, f32)) -> Event {
        let _target = self.get_mouse_target();

        // TODO: just like ScrollBy/ScrollAt update message (& render() after that)
        //self.renderer.scroll(self.mouse_pos, delta);

        Event::Scroll { target: self.get_mouse_target() }
    }
    */

    pub fn mouse_down(&mut self) -> Event {
        Event::MouseDown { target: self.get_mouse_target() }
    }

    pub fn mouse_up(&mut self) -> Event {
        Event::MouseUp { target: self.get_mouse_target() }
    }

    pub fn key_down(&mut self, scancode: u16) -> Event {
        Event::KeyDown { target: 0, key: scancode }
    }

    pub fn key_press(&mut self, char: u16) -> Event {
        Event::KeyPress { target: 0, key: char }
    }

    pub fn key_up(&mut self, scancode: u16) -> Event {
        Event::KeyUp { target: 0, key: scancode }
    }

    pub fn resize(&mut self, size: (f32, f32)) -> Event {
        self.size = size;

        self.renderer.resize(size.0, size.1);

        // make screen overflow (for now)
        // TODO: overflow: scroll could fix it
        // @see calculate() in `yoga.rs`
        self.set_style(Self::ROOT, &StyleProp::MinHeight(Dimension::Px(size.1)));

        Event::Resize { target: Self::ROOT }
    }

    pub fn close(&mut self) -> Event {
        Event::Close { target: Self::ROOT }
    }
}

pub type NodeId = usize;
