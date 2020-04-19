// x owns the systems
// x connect/orchestrate them together
//   x renderer needs bounds from layout
//   x layout needs measure from text
//   x font/size/* should propagate to text nodes
//     x when text node is inserted/removed
//     x when el text style is changed

use crate::box_layout::{Align, BoxLayoutImpl, BoxLayoutTree, Dimension, Display, FlexDirection, FlexWrap, Overflow};
use crate::commons::{Bounds, Id, Pos};
use crate::render::backend::gl::GlRenderBackend;
use crate::render::backend::{BackendOp, FillStyle, RenderBackend};
use crate::render::value_types::{BorderStyle, Color};
use crate::render::{Renderer, SurfaceId};
use crate::text::{TextAlign, TextEngine, TextId, TextStyle};

pub type GlViewport = Viewport<GlRenderBackend>;

pub struct Viewport<RB: RenderBackend> {
    size: (f32, f32),

    // systems
    layout_engine: BoxLayoutImpl,
    text_engine: TextEngine,
    renderer: Renderer<RB, <BoxLayoutImpl as BoxLayoutTree>::LayoutNodeId>,

    // nodes
    nodes: Vec<Node>,
    elements: Vec<Element>,
    text_nodes: Vec<TextNode>,

    // flags
    needs_layout: bool,

    // TODO: fullscreen + track the element id
    // (and use it in render/calculate)
    mouse_pos: Pos,
}

// text nodes can receive events too and
// it might be useful for text-selection
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
#[derive(Debug, Clone)]
pub enum StyleProp {
    // TODO: accept Dimension in non-layout props (this is big)
    // TODO: border colors
    // TODO: FontStyle, FontVariant
    AlignContent(Align),
    AlignItems(Align),
    AlignSelf(Align),
    BackgroundColor(Color),
    BorderBottomLeftRadius(f32),
    BorderBottomRightRadius(f32),
    //BorderBottomStyle(BorderStyle),
    BorderBottomWidth(f32),
    //BorderLeftStyle(BorderStyle),
    BorderLeftWidth(f32),
    //BorderRightStyle(BorderStyle),
    BorderRightWidth(f32),
    BorderTopLeftRadius(f32),
    BorderTopRightRadius(f32),
    //BorderTopStyle(BorderStyle),
    BorderTopWidth(f32),
    Bottom(Dimension),
    Color(Color),
    Display(Display),
    FlexBasis(Dimension),
    FlexDirection(FlexDirection),
    FlexGrow(f32),
    FlexShrink(f32),
    FlexWrap(FlexWrap),
    FontFamily(String),
    FontSize(f32),
    Height(Dimension),
    JustifyContent(Align),
    Left(Dimension),
    LineHeight(f32),
    MarginBottom(Dimension),
    MarginLeft(Dimension),
    MarginRight(Dimension),
    MarginTop(Dimension),
    MaxHeight(Dimension),
    MaxWidth(Dimension),
    MinHeight(Dimension),
    MinWidth(Dimension),
    Overflow(Overflow),
    PaddingBottom(Dimension),
    PaddingLeft(Dimension),
    PaddingRight(Dimension),
    PaddingTop(Dimension),
    Right(Dimension),
    TextAlign(TextAlign),
    Top(Dimension),
    Width(Dimension),
}

impl<RB: RenderBackend> Viewport<RB> {
    pub const ROOT: NodeId = NodeId::new(0);

    pub fn new(backend: RB, size: (f32, f32)) -> Viewport<RB> {
        let mut viewport = Viewport {
            size,

            layout_engine: BoxLayoutImpl::new(),
            text_engine: TextEngine::new(),
            renderer: Renderer::new(backend),

            nodes: Vec::new(),
            elements: Vec::new(),
            text_nodes: Vec::new(),

            needs_layout: false,

            mouse_pos: Pos::ZERO,
        };

        // create root
        viewport.create_element();

        // set min-height
        viewport.resize(size);

        viewport
    }

    pub fn create_element(&mut self) -> NodeId {
        let id = NodeId::new(self.nodes.len());
        let element_id = ElementId::new(self.elements.len());

        let layout = self.layout_engine.create_node(None);
        let surface = self.renderer.create_surface(layout);
        let text_nodes = Vec::new();
        let text_style = TextStyle {
            font: TextEngine::DEFAULT_FONT,
            font_size: 16.,
            line_height: 30.,
            align: TextAlign::Left,
        };

        self.elements.push(Element {
            layout,
            surface,
            text_nodes,
            text_style,
        });
        self.nodes.push(Node::Element(element_id));

        id
    }

    pub fn create_text_node(&mut self) -> NodeId {
        let id = NodeId::new(self.nodes.len());
        let text_node_id = TextNodeId::new(self.text_nodes.len());

        let text = self.text_engine.create_text();
        let layout = self.layout_engine.create_node(Some(text.0));
        let surface = self.renderer.create_surface(layout);
        let prev_width = 0.;

        self.text_nodes.push(TextNode { text, layout, surface, prev_width });
        self.nodes.push(Node::TextNode(text_node_id));

        id
    }

    pub fn insert_child(&mut self, element: NodeId, index: usize, child: NodeId) {
        self.needs_layout = true;

        let el_id = self.element_id(element);
        let child_layout = self.node_layout(child);
        let child_surface = self.node_surface(child);

        let el = &mut self.elements[el_id];

        self.layout_engine.insert_child(el.layout, index, child_layout);
        self.renderer.insert_child(el.surface, index, child_surface);

        if let Some(tn_id) = self.nodes[child].text_node_id() {
            let tn = &mut self.text_nodes[tn_id];

            // order doesn't matter
            el.text_nodes.push(tn_id);

            self.text_engine.set_text_style(tn.text, &el.text_style);
            self.mark_dirty(tn_id);
        }
    }

    pub fn remove_child(&mut self, element: NodeId, child: NodeId) {
        self.needs_layout = true;

        let el_id = self.element_id(element);
        let child_layout = self.node_layout(child);
        let child_surface = self.node_surface(child);

        let el = &mut self.elements[el_id];

        if let Some(tn_id) = self.nodes[child].text_node_id() {
            el.text_nodes.retain(|ch| *ch != tn_id);
        }

        self.layout_engine.remove_child(el.layout, child_layout);
        self.renderer.remove_child(el.surface, child_surface);
    }

    // TODO: accept & store dyn IntoIterator<Item = char>
    //       so we can store actual (persistent) V8 strings (impl !Send)
    //       and save some memory
    pub fn set_text(&mut self, text_node: NodeId, text: String) {
        self.needs_layout = true;

        let text_node = self.text_node_id(text_node);

        self.text_engine.set_text_chars(self.text_nodes[text_node].text, text);
        self.mark_dirty(text_node);
    }

    pub fn set_style(&mut self, element: NodeId, prop: &StyleProp) {
        use StyleProp::*;

        let el = self.element_id(element);

        match prop {
            // TODO: Border
            // might need relayout!
            prop => {
                match prop {
                    // layout-independent
                    BorderTopLeftRadius(_) | BorderTopRightRadius(_) | BorderBottomRightRadius(_) | BorderBottomLeftRadius(_) | Color(_) | BackgroundColor(_) => {
                        self.set_surface_style(el, prop);
                    }

                    // layout-only
                    Width(_) | Height(_) | MinWidth(_) | MinHeight(_) | MaxWidth(_) | MaxHeight(_) | Top(_) | Right(_) | Bottom(_) | Left(_) | MarginTop(_) | MarginRight(_) | MarginBottom(_)
                    | MarginLeft(_) | PaddingTop(_) | PaddingRight(_) | PaddingBottom(_) | PaddingLeft(_) | FlexGrow(_) | FlexShrink(_) | FlexBasis(_) | FlexDirection(_) | FlexWrap(_)
                    | AlignSelf(_) | AlignContent(_) | AlignItems(_) | JustifyContent(_) => {
                        self.set_layout_style(el, prop);
                    }

                    // both
                    Display(_) | Overflow(_) | BorderTopWidth(_) | BorderRightWidth(_) | BorderBottomWidth(_) | BorderLeftWidth(_) => {
                        self.set_surface_style(el, prop);
                        self.set_layout_style(el, prop);
                    }

                    // text
                    FontFamily(_) | FontSize(_) | LineHeight(_) | TextAlign(_) => {
                        self.set_text_style(el, prop);
                    }
                }
            }
        }
    }

    fn set_surface_style(&mut self, element: ElementId, prop: &StyleProp) {
        let r = &mut self.renderer;
        let s = self.elements[element].surface;

        use StyleProp::*;
        match prop {
            // TODO: display: none + visibility: hidden
            Display(_)
            | Overflow(_)
            | BorderTopLeftRadius(_)
            | BorderTopRightRadius(_)
            | BorderBottomRightRadius(_)
            | BorderBottomLeftRadius(_)
            | BorderTopWidth(_)
            | BorderRightWidth(_)
            | BorderBottomWidth(_)
            | BorderLeftWidth(_) => {}

            // TODO: Transform
            // TODO: BorderRadius
            // TODO: BoxShadow
            // TODO: Image
            Color(v) => r.set_color(s, *v),
            BackgroundColor(v) => r.set_background_color(s, *v),

            _ => unreachable!(),
        }
    }

    fn set_layout_style(&mut self, element: ElementId, prop: &StyleProp) {
        self.needs_layout = true;

        let le = &mut self.layout_engine;
        let l = self.elements[element].layout;

        use StyleProp::*;
        match prop {
            Display(v) => le.set_display(l, *v),

            Width(v) => le.set_width(l, *v),
            Height(v) => le.set_height(l, *v),
            MinWidth(v) => le.set_min_width(l, *v),
            MinHeight(v) => le.set_min_height(l, *v),
            MaxWidth(v) => le.set_max_width(l, *v),
            MaxHeight(v) => le.set_max_height(l, *v),

            Top(v) => le.set_top(l, *v),
            Right(v) => le.set_right(l, *v),
            Bottom(v) => le.set_bottom(l, *v),
            Left(v) => le.set_left(l, *v),

            MarginTop(v) => le.set_margin_top(l, *v),
            MarginRight(v) => le.set_margin_right(l, *v),
            MarginBottom(v) => le.set_margin_bottom(l, *v),
            MarginLeft(v) => le.set_margin_left(l, *v),

            PaddingTop(v) => le.set_padding_top(l, *v),
            PaddingRight(v) => le.set_padding_right(l, *v),
            PaddingBottom(v) => le.set_padding_bottom(l, *v),
            PaddingLeft(v) => le.set_padding_left(l, *v),

            FlexGrow(v) => le.set_flex_grow(l, *v),
            FlexShrink(v) => le.set_flex_shrink(l, *v),
            FlexBasis(v) => le.set_flex_basis(l, *v),
            FlexDirection(v) => le.set_flex_direction(l, *v),
            FlexWrap(v) => le.set_flex_wrap(l, *v),

            AlignSelf(v) => le.set_align_self(l, *v),
            AlignContent(v) => le.set_align_content(l, *v),
            AlignItems(v) => le.set_align_items(l, *v),
            JustifyContent(v) => le.set_justify_content(l, *v),

            BorderTopWidth(v) => le.set_border_top(l, *v),
            BorderRightWidth(v) => le.set_border_right(l, *v),
            BorderBottomWidth(v) => le.set_border_bottom(l, *v),
            BorderLeftWidth(v) => le.set_border_left(l, *v),

            _ => unreachable!(),
        }
    }

    fn set_text_style(&mut self, element: ElementId, prop: &StyleProp) {
        self.needs_layout = true;

        let el = &mut self.elements[element];
        let ts = &mut el.text_style;

        use StyleProp::*;
        match prop {
            FontFamily(v) => ts.font = self.text_engine.resolve_font(v),
            FontSize(v) => ts.font_size = *v,
            LineHeight(v) => ts.line_height = *v,
            TextAlign(v) => ts.align = *v,

            _ => unreachable!(),
        }

        for tn_id in &el.text_nodes {
            let tn = &mut self.text_nodes[*tn_id];

            self.text_engine.set_text_style(tn.text, ts);
            self.layout_engine.mark_dirty(tn.layout);

            tn.prev_width = 0.;
        }
    }

    fn mark_dirty(&mut self, text_node: TextNodeId) {
        let tn = &mut self.text_nodes[text_node];

        self.layout_engine.mark_dirty(tn.layout);

        // set to 0. so the glyphs are invalidated and uploaded during `update()`
        tn.prev_width = 0.;
    }

    // has to be called before querying!
    pub fn update(&mut self) {
        silly!("update/render");

        if self.needs_layout {
            self.update_layout();
            self.update_texts();

            self.needs_layout = false;
        }

        // TODO: split preparation and rendering so it's thread-friendly
        let layout_engine = &self.layout_engine;

        self.renderer.render_surface(self.elements[Self::ROOT_ELEMENT].surface, &|k| layout_engine.get_bounds(k));
    }

    fn update_layout(&mut self) {
        let Self { text_engine, .. } = self;

        // TODO: replace with Lookup<(id, w), (f32, f32)>
        self.layout_engine.calculate(self.elements[Self::ROOT_ELEMENT].layout, self.size, &mut |text_id, max_width| {
            text_engine.measure_text(TextId::new(text_id), max_width)
        });
    }

    fn update_texts(&mut self) {
        // there's no guarantee `measure()` will be called so we need to
        // find out if the `max_width` changed and rebuild such text nodes
        //
        // we could check/rebuild caches during render but that would need
        // to be sync and it's likely the CPU wouldn't like that switching either
        // this should be fast enough or maybe even faster
        //
        // TODO: run in parallel and/or concurrently with
        //       the rest of the render preparation

        for tn in &mut self.text_nodes {
            let w = self.layout_engine.get_width(tn.layout);

            if w != tn.prev_width {
                self.renderer.set_content(
                    tn.surface,
                    Some(self.text_engine.get_text_glyphs(tn.text, w).map(|g| {
                        BackendOp::PushRect(g.bounds, FillStyle::SolidColor(Color::RED))
                    })),
                );

                tn.prev_width = w;
            }
        }
    }

    pub fn get_offset_bounds(&self, element: NodeId) -> Bounds {
        self.layout_engine.get_bounds(self.elements[self.element_id(element)].layout)
    }

    fn get_mouse_target(&self) -> NodeId {
        Self::ROOT
        //self.picker.pick_at(self.mouse_pos, &self.children, &self.layout_engine)
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
        Event::KeyDown {
            target: Self::NO_TARGET,
            key: scancode,
        }
    }

    pub fn key_press(&mut self, char: u16) -> Event {
        Event::KeyPress { target: Self::NO_TARGET, key: char }
    }

    pub fn key_up(&mut self, scancode: u16) -> Event {
        Event::KeyUp {
            target: Self::NO_TARGET,
            key: scancode,
        }
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

    // helpers

    fn element_id(&self, node: NodeId) -> ElementId {
        self.nodes[node].element_id().expect("not an element")
    }

    fn text_node_id(&self, node: NodeId) -> TextNodeId {
        self.nodes[node].text_node_id().expect("not a text node")
    }

    fn node_layout(&self, node: NodeId) -> <BoxLayoutImpl as BoxLayoutTree>::LayoutNodeId {
        match self.nodes[node] {
            Node::Element(id) => self.elements[id].layout,
            Node::TextNode(id) => self.text_nodes[id].layout,
        }
    }

    fn node_surface(&self, node: NodeId) -> SurfaceId {
        match self.nodes[node] {
            Node::Element(id) => self.elements[id].surface,
            Node::TextNode(id) => self.text_nodes[id].surface,
        }
    }

    const ROOT_ELEMENT: ElementId = ElementId::new(0);

    // TODO: either send target or remove it from variants
    const NO_TARGET: NodeId = NodeId::new(0);
}

pub type NodeId = Id<Node>;

// private from here
// (pub is needed because of Id<T>)

// keep separately so we can quickly rebuild texts
// without having to skip elements
pub enum Node {
    Element(ElementId),
    TextNode(TextNodeId),
}

impl Node {
    fn element_id(&self) -> Option<ElementId> {
        match self {
            Node::Element(id) => Some(*id),
            _ => None,
        }
    }

    fn text_node_id(&self) -> Option<TextNodeId> {
        match self {
            Node::TextNode(id) => Some(*id),
            _ => None,
        }
    }
}

type ElementId = Id<Element>;
type TextNodeId = Id<TextNode>;

pub struct Element {
    layout: <BoxLayoutImpl as BoxLayoutTree>::LayoutNodeId,
    surface: SurfaceId,

    text_nodes: Vec<TextNodeId>,
    text_style: TextStyle,
}

pub struct TextNode {
    layout: <BoxLayoutImpl as BoxLayoutTree>::LayoutNodeId,
    surface: SurfaceId,

    text: TextId,
    prev_width: f32,
}
