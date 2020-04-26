// x owns the systems
// x connect/orchestrate them together
//   x renderer needs bounds from layout
//   x layout needs measure from text
//   x font/size/* should propagate to text nodes
//     x when text node is inserted/removed
//     x when el text style is changed

use crate::box_layout::{BoxLayoutImpl, BoxLayoutTree, Dimension};
use crate::commons::{Bounds, Id, Pos};
use crate::render::backend::gl::GlRenderBackend;
use crate::render::backend::RenderBackend;
use crate::render::{Renderer, SurfaceId};
use crate::style::StyleProp;
use crate::text::{TextAlign, TextEngine, TextId, TextStyle};

pub type GlViewport = Viewport<GlRenderBackend>;

pub struct Viewport<RB: RenderBackend> {
    size: (f32, f32),

    // nodes
    node_data: Vec<NodeData>,
    layouts: Vec<<BoxLayoutImpl as BoxLayoutTree>::LayoutNodeId>,
    surfaces: Vec<SurfaceId>,

    // systems
    layout_engine: BoxLayoutImpl,
    text_engine: TextEngine,
    renderer: Renderer<RB, <BoxLayoutImpl as BoxLayoutTree>::LayoutNodeId>,

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

impl<RB: RenderBackend> Viewport<RB> {
    pub const ROOT: NodeId = NodeId::new(0);

    pub fn new(backend: RB, size: (f32, f32)) -> Viewport<RB> {
        let mut viewport = Viewport {
            size,

            node_data: Vec::new(),
            layouts: Vec::new(),
            surfaces: Vec::new(),

            layout_engine: BoxLayoutImpl::new(),
            text_engine: TextEngine::new(),
            renderer: Renderer::new(backend),

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
        self.push_node(NodeData::Element(ElementData {
            children: Vec::new(),
            text_style: TextStyle {
                font: TextEngine::DEFAULT_FONT,
                font_size: 16.,
                line_height: 30.,
                align: TextAlign::Left,
            },
        }))
    }

    pub fn create_text_node(&mut self) -> NodeId {
        let text = self.text_engine.create_text();

        self.push_node(NodeData::TextNode(TextNodeData { text, prev_width: 0. }))
    }

    fn push_node(&mut self, node_data: NodeData) -> NodeId {
        let layout = self.layout_engine.create_node(match node_data {
            NodeData::Element(_) => None,
            NodeData::TextNode(TextNodeData { text, .. }) => Some(text.0),
        });
        let surface = self.renderer.create_surface(layout);

        self.node_data.push(node_data);
        self.layouts.push(layout);
        self.surfaces.push(surface);

        NodeId::new(self.node_data.len() - 1)
    }

    pub fn insert_child(&mut self, element: NodeId, index: usize, child: NodeId) {
        self.node_data[element].el().children.insert(index, child);

        // layout
        self.needs_layout = true;
        self.layout_engine.insert_child(self.layouts[element.0], index, self.layouts[child.0]);

        // renderer
        self.update_el_surfaces(element);

        // text
        if let NodeData::TextNode(TextNodeData { text, .. }) = self.node_data[child] {
            let ts = self.node_data[element].el().text_style;
            self.text_engine.set_text_style(text, &ts);
            self.mark_dirty(child);
        }
    }

    pub fn remove_child(&mut self, element: NodeId, child: NodeId) {
        self.node_data[element].el().children.retain(|ch| *ch != child);

        // layout
        self.needs_layout = true;
        self.layout_engine.remove_child(self.layouts[element.0], self.layouts[child.0]);

        // renderer
        self.update_el_surfaces(element);
    }

    fn update_el_surfaces(&mut self, element: NodeId) {
        let Self { node_data, surfaces, .. } = self;

        let child_surfaces: Vec<SurfaceId> = node_data[element].el().children.iter().copied().map(|ch| surfaces[ch.0]).collect();
        self.renderer.set_children(surfaces[element.0], &child_surfaces);
    }

    // TODO: accept & store dyn Iterator<Item = &char>
    //       so we can store actual (persistent) V8 strings (impl !Send)
    //       and save some memory
    //       (iter.copied()...)
    pub fn set_text(&mut self, text_node: NodeId, text: String) {
        self.needs_layout = true;

        self.text_engine.set_text_chars(self.node_data[text_node].tn().text, text);
        self.mark_dirty(text_node);
    }

    pub fn set_style(&mut self, element: NodeId, prop: &StyleProp) {
        use StyleProp::*;

        match prop {
            // layout-independent
            BorderTopLeftRadius(_) | BorderTopRightRadius(_) | BorderBottomRightRadius(_) | BorderBottomLeftRadius(_) | Color(_) | BackgroundColor(_) => {
                self.set_surface_style(element, prop);
            }

            // layout-only
            Width(_) | Height(_) | MinWidth(_) | MinHeight(_) | MaxWidth(_) | MaxHeight(_) | Top(_) | Right(_) | Bottom(_) | Left(_) | MarginTop(_) | MarginRight(_) | MarginBottom(_)
            | MarginLeft(_) | PaddingTop(_) | PaddingRight(_) | PaddingBottom(_) | PaddingLeft(_) | FlexGrow(_) | FlexShrink(_) | FlexBasis(_) | FlexDirection(_) | FlexWrap(_) | AlignSelf(_)
            | AlignContent(_) | AlignItems(_) | JustifyContent(_) => {
                self.set_layout_style(element, prop);
            }

            // both
            Display(_) | Overflow(_) | BorderTopWidth(_) | BorderRightWidth(_) | BorderBottomWidth(_) | BorderLeftWidth(_) => {
                self.set_surface_style(element, prop);
                self.set_layout_style(element, prop);
            }

            // text
            FontFamily(_) | FontSize(_) | LineHeight(_) | TextAlign(_) => {
                self.set_text_style(element, prop);
            }
        }
    }

    fn set_surface_style(&mut self, element: NodeId, prop: &StyleProp) {
        let r = &mut self.renderer;
        let s = self.surfaces[element.0];

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

    fn set_layout_style(&mut self, element: NodeId, prop: &StyleProp) {
        self.needs_layout = true;

        let le = &mut self.layout_engine;
        let l = self.layouts[element.0];

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

    fn set_text_style(&mut self, element: NodeId, prop: &StyleProp) {
        /*
        self.needs_layout = true;

        let ElementData { text_style, children } = self.node_data[element].el();

        use StyleProp::*;
        match prop {
            FontFamily(v) => text_style.font = self.text_engine.resolve_font(v),
            FontSize(v) => text_style.font_size = *v,
            LineHeight(v) => text_style.line_height = *v,
            TextAlign(v) => text_style.align = *v,

            _ => unreachable!(),
        }

        for ch in children {
            if let NodeData::TextNode(TextNodeData { text, prev_width }) = self.node_data[ch.0] {
                self.text_engine.set_text_style(text, text_style);
                self.layout_engine.mark_dirty(self.layouts[ch.0]);

                prev_width = 0.;
            }
        }
        */
    }

    fn mark_dirty(&mut self, text_node: NodeId) {
        self.layout_engine.mark_dirty(self.layouts[text_node.0]);

        // set to 0. so the glyphs are invalidated and uploaded during `update()`
        self.node_data[text_node].tn().prev_width = 0.;
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

        self.renderer.render_surface(self.surfaces[Self::ROOT.0], &|k| layout_engine.get_bounds(k));
    }

    fn update_layout(&mut self) {
        let Self { text_engine, .. } = self;

        // TODO: replace with Lookup<(id, w), (f32, f32)>
        self.layout_engine.calculate(self.layouts[Self::ROOT.0], self.size, &mut |text_id, max_width| {
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
        //       (put prev_width in own vec to avoid cache invalidation in other threads)
        //
        // TODO: maybe it could be moved to TextEngine
        //       (and so only text nodes could be visited)

        for (i, data) in &mut self.node_data.iter_mut().enumerate() {
            if let NodeData::TextNode(TextNodeData { text, prev_width }) = data {
                let w = self.layout_engine.get_width(self.layouts[i]);

                // TODO: wont work
                let texture = self.renderer.create_texture(512, 512);

                if w != *prev_width {
                    self.renderer
                        .set_text(self.surfaces[i], texture, self.text_engine.get_text_glyphs(*text, w).map(|g| (g.bounds, Bounds::ZERO_ONE)));

                    *prev_width = w;
                }
            }
        }
    }

    pub fn get_offset_bounds(&self, element: NodeId) -> Bounds {
        self.layout_engine.get_bounds(self.layouts[element.0])
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

    // TODO: either send target or remove it from variants
    const NO_TARGET: NodeId = NodeId::new(0);
}

pub type NodeId = Id<NodeData>;

// private from here
// (pubs because of Id<T>)

pub enum NodeData {
    Element(ElementData),
    TextNode(TextNodeData),
}

impl NodeData {
    fn el(&mut self) -> &mut ElementData {
        match self {
            NodeData::Element(data) => data,
            _ => panic!("not an element"),
        }
    }

    fn tn(&mut self) -> &mut TextNodeData {
        match self {
            NodeData::TextNode(data) => data,
            _ => panic!("not a text node"),
        }
    }
}

pub struct ElementData {
    children: Vec<NodeId>,
    text_style: TextStyle,
}

pub struct TextNodeData {
    text: TextId,
    prev_width: f32,
}
