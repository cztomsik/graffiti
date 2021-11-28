// TODO: this is likely not final design either, I don't like that we always traverse & interpret whole tree
//       and I think it should be possible to have something like retained-mesh design (create_mesh() -> Id),
//       where every mesh/model is updated in .update() and render() can just (sequentially) draw those
//       which would be hell more complicated so I'm keeping it for later

#![allow(unused)]

use crate::css::{
    CssAlign, CssDimension, CssDisplay, CssFlexDirection, CssFlexWrap, CssJustify, CssStyleSheet, StyleProp,
    StyleResolver,
};
use crate::gfx::{Canvas, GlBackend, PathCmd, RenderBackend, Text, TextStyle, Transform, Vec2, AABB, RGBA8};
use crate::layout::{
    Align, Dimension, Display, FlexDirection, FlexWrap, Justify, LayoutNodeId, LayoutStyle, LayoutTree, Size,
};
use crate::util::{BitSet, Edge, SlotMap};
use crate::{DocumentRef, DomEvent, ElementRef, NodeId, NodeRef, NodeType, Window};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

pub struct Renderer {
    document: DocumentRef,
    style_resolver: StyleResolver,
    state: Rc<RefCell<State>>,
    listener: Rc<dyn Fn(&DomEvent)>,
    window: Arc<Window>,
    backend: GlBackend,
}

#[derive(Default)]
struct State {
    layout_tree: LayoutTree,
    render_nodes: SlotMap<NodeId, RenderNode>,
    dirty_nodes: BitSet,
}

impl Renderer {
    pub fn new(document: &DocumentRef, win: &Window) -> Self {
        // TODO: make this whole fn unsafe?
        let backend = unsafe { GlBackend::new(|s| win.get_proc_address(s) as _) };

        let state = Rc::new(RefCell::new(State::default()));
        let listener = Self::create_listener(Rc::clone(&state));

        // connect
        document.add_listener(Rc::clone(&listener));
        for node in document.all_nodes() {
            listener(&DomEvent::NodeCreated(&node));
        }
        for parent in document.all_nodes() {
            for child in parent.child_nodes() {
                listener(&DomEvent::AppendChild(&parent, &child));
            }
        }

        Self {
            window: Window::find_by_id(win.id()).unwrap(),
            document: DocumentRef::clone(document),
            style_resolver: StyleResolver::new(vec![Rc::new(CssStyleSheet::default_ua_sheet())]),
            state,
            listener,
            backend,
        }
    }

    fn create_listener(state: Rc<RefCell<State>>) -> Rc<dyn Fn(&DomEvent)> {
        fn mark_dirty(dirty_nodes: &mut BitSet, node: &NodeRef) {
            // recurse but stop early
            if !dirty_nodes.contains(node.id()) {
                dirty_nodes.add(node.id());

                for ch in node.child_nodes() {
                    mark_dirty(dirty_nodes, &ch);
                }
            }
        }

        Rc::new(move |event| {
            let State {
                layout_tree,
                render_nodes,
                dirty_nodes,
                ..
            } = &mut *state.borrow_mut();
            match event {
                DomEvent::NodeCreated(node) => {
                    render_nodes.put(
                        node.id(),
                        RenderNode {
                            dom_node: NodeRef::clone(node),
                            layout_node: layout_tree.create_node(),
                        },
                    );
                    dirty_nodes.grow(node.id());
                }
                &DomEvent::NodeDestroyed(id) => {
                    layout_tree.drop_node(render_nodes.remove(id).unwrap().layout_node);
                    // if whole subtree gets freed, it might not be removed at all
                    dirty_nodes.remove(id);
                }
                DomEvent::AppendChild(parent, child) => {
                    layout_tree.append_child(
                        render_nodes[parent.id()].layout_node,
                        render_nodes[child.id()].layout_node,
                    );
                    mark_dirty(dirty_nodes, child);
                }
                DomEvent::InsertBefore(parent, child, before) => {
                    layout_tree.insert_before(
                        render_nodes[parent.id()].layout_node,
                        render_nodes[child.id()].layout_node,
                        render_nodes[before.id()].layout_node,
                    );
                    dirty_nodes.add(child.id());
                }
                DomEvent::RemoveChild(parent, child) => {
                    layout_tree.remove_child(
                        render_nodes[parent.id()].layout_node,
                        render_nodes[child.id()].layout_node,
                    );
                    dirty_nodes.remove(child.id());
                }
            }
        })
    }

    pub fn render(&self) {
        self.update();

        let state = self.state.borrow();

        let mut canvas = Canvas::new();
        let mut ctx = RenderContext {
            canvas: &mut canvas,
            render_nodes: &state.render_nodes,
            layout_tree: &state.layout_tree,
        };

        profile!();
        ctx.render_node(Vec2::new(0., 0.), self.document.id());
        let frame = ctx.canvas.flush();
        profile!("frame");

        unsafe { self.window.make_current() };
        self.backend.render_frame(frame);
        self.window.swap_buffers();
        profile!("gl + vsync");
    }

    pub fn resize(&self, width: f32, height: f32) {
        println!("TODO: Renderer::resize({}, {})", width, height);
    }

    pub fn update(&self) {
        profile!();

        let State {
            layout_tree,
            render_nodes,
            dirty_nodes,
        } = &mut *self.state.borrow_mut();

        let sheets: Vec<_> = self
            .document
            .query_selector_all("style")
            .iter()
            .map(|s| s.text_content())
            .filter_map(|s| CssStyleSheet::parse(&s).ok())
            .collect();

        for id in dirty_nodes.iter() {
            let node = self.document.find_node(id).unwrap();
            if let Some(el) = node.as_element() {
                println!("update style {:?}", (el.local_name(), id));

                let mut res = self.style_resolver.resolve_style(&el, ResolvedStyle::apply_style_prop);
                for p in el.style().props().iter() {
                    res.apply_style_prop(p);
                }

                layout_tree.set_style(render_nodes[id].layout_node, res.layout_style);
                println!("TODO: set render style");
                //render_styles.put(id, res.render_style);
            } else if let Some(text) = node.as_text() {
                println!("TODO: update text/comment");
            } else if let Some(doc) = node.as_document() {
                layout_tree.set_style(
                    render_nodes[id].layout_node,
                    LayoutStyle {
                        display: Display::Block,
                        ..Default::default()
                    },
                )
            }
        }

        dirty_nodes.clear();
        profile!("css");

        layout_tree.calculate(render_nodes[self.document.id()].layout_node, 1024., 768.);
        profile!("layout");
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.document.remove_listener(&self.listener);
    }
}

struct RenderContext<'a> {
    canvas: &'a mut Canvas,
    render_nodes: &'a SlotMap<NodeId, RenderNode>,
    layout_tree: &'a LayoutTree,
}

impl<'a> RenderContext<'a> {
    fn draw_bg_color(&mut self, rect: AABB, radii: [f32; 4], color: RGBA8) {
        if radii == [0., 0., 0., 0.] {
            return self.canvas.fill_rect(rect, color);
        }

        let Vec2 { x, y } = rect.min;
        let [w, h] = [rect.max.x - rect.min.x, rect.max.y - rect.min.y];
        let [tl, tr, br, bl] = radii;

        // TODO: clamping (when r > w/h)
        let path = [
            PathCmd::Move(Vec2::new(x + tl, y)),
            PathCmd::Line(Vec2::new(x + w - tr, y)),
            PathCmd::Quadratic(Vec2::new(x + w, y), Vec2::new(x + w, y + tr)),
            PathCmd::Line(Vec2::new(x + w, y + h - br)),
            PathCmd::Quadratic(Vec2::new(x + w, y + h), Vec2::new(x + w - br, y + h)),
            PathCmd::Line(Vec2::new(x + bl, y + h)),
            PathCmd::Quadratic(Vec2::new(x + w, y + h), Vec2::new(x + w - br, y + h)),
            PathCmd::Line(Vec2::new(x + bl, y + h)),
            PathCmd::Quadratic(Vec2::new(x, y + h), Vec2::new(x, y + h - bl)),
            PathCmd::Line(Vec2::new(x, y + tl)),
            PathCmd::Quadratic(Vec2::new(x, y), Vec2::new(x + tl, y)),
            PathCmd::Close,
        ];

        self.canvas.fill_path(&path, color);
    }


        let rect = res.outer_rect();
        let pos = Vec2::new(rect.left, rect.top);
        let rect = AABB::new(pos, Vec2::new(rect.right, rect.bottom));

        // if let Some(text) = &layout_box.text {
        //     // TODO: skip in layout?
        //     if layout_box.width() > 0. {
        //         self.canvas.fill_text(text, rect, [0, 0, 0, 255]);
        //     }
        // } else {
        self.canvas.fill_rect(rect, [255, 0, 0, 30]);
        // }

        for ch in self.layout_tree.children(layout_node) {
            self.render_box(pos.x, pos.y, ch);
        }
    }

    /*
    fn render_element(&mut self, rect: AABB, style: &RenderStyle, children: impl Iterator<Item = NodeId>) {
        if style.hidden {
            return;
        }

        // TODO: border_radius, clip, scroll, opacity, transform, ...

        // TODO: outline shadow(s)

        if let Some((width, color)) = style.outline {
            self.render_outline(rect, width, color);
        }

        if let Some(bg_color) = style.bg_color {
            self.canvas.fill_rect(rect, bg_color);
        }

        // TODO: image(s)

        // TODO: inset shadow(s)

        for ch in children {
            self.render_node(rect.min, ch);
        }

        // TODO: border
    }

    fn render_outline(&mut self, rect: AABB, width: f32, color: RGBA8) {
        let AABB { min, max } = rect;

        // top
        self.canvas.fill_rect(
            AABB::new(min - Vec2::new(width, width), Vec2::new(max.x + width, min.y)),
            color,
        );

        // right
        self.canvas.fill_rect(
            AABB::new(Vec2::new(max.x, min.y), Vec2::new(max.x + width, max.y)),
            color,
        );

        // bottom
        self.canvas.fill_rect(
            AABB::new(Vec2::new(min.x - width, max.y), max + Vec2::new(width, width)),
            color,
        );

        // left
        self.canvas.fill_rect(
            AABB::new(Vec2::new(min.x - width, min.y), Vec2::new(min.x, max.y)),
            color,
        );
    }
    */
}

struct RenderNode {
    dom_node: NodeRef,
    layout_node: LayoutNodeId,
}

#[derive(Default)]
struct ResolvedStyle {
    layout_style: LayoutStyle,
    // text_style: TextStyle,
    render_style: RenderStyle,
}

struct RenderStyle {
    transform: Transform,
    //overflow: visible/hidden/scroll,
    opacity: f32,
    border_radii: [f32; 4],
    // - outline_shadow(s)
    //outline: (f32, /*CssBorderStyle,*/ RGBA8),
    // - clip() from here if overflow hidden
    bg_color: RGBA8,
    // - bg_image(s) | gradient(s)
    // - inset_shadow(s)
    // - children
    // - border
}

impl Default for RenderStyle {
    fn default() -> Self {
        Self {
            transform: Transform::id(),
            opacity: 1.,
            border_radii: [0., 0., 0., 0.],
            bg_color: [0, 0, 0, 0],
        }
    }
}

impl ResolvedStyle {
    // fn apply_style(&mut self, style: &CssStyleDeclaration) {}

    fn apply_style_prop(&mut self, prop: &StyleProp) {
        use StyleProp::*;
        match prop {
            // first, likely to be animated
            &Opacity(v) => self.render_style.opacity = v,
            &BackgroundColor(v) => self.render_style.bg_color = [v.r, v.g, v.b, v.a],

            // size
            &Width(v) => self.layout_style.width = dimension(v),
            &Height(v) => self.layout_style.height = dimension(v),
            &MinWidth(v) => self.layout_style.min_width = dimension(v),
            &MinHeight(v) => self.layout_style.min_height = dimension(v),
            &MaxWidth(v) => self.layout_style.max_width = dimension(v),
            &MaxHeight(v) => self.layout_style.max_height = dimension(v),

            // padding
            &PaddingTop(v) => self.layout_style.padding_top = dimension(v),
            &PaddingRight(v) => self.layout_style.padding_right = dimension(v),
            &PaddingBottom(v) => self.layout_style.padding_bottom = dimension(v),
            &PaddingLeft(v) => self.layout_style.padding_left = dimension(v),

            // margin
            &MarginTop(v) => self.layout_style.margin.top = dimension(v),
            &MarginRight(v) => self.layout_style.margin.right = dimension(v),
            &MarginBottom(v) => self.layout_style.margin.bottom = dimension(v),
            &MarginLeft(v) => self.layout_style.margin.left = dimension(v),

            // border
            &BorderTopWidth(v) => self.layout_style.border.top = dimension(v),
            &BorderRightWidth(v) => self.layout_style.border.right = dimension(v),
            &BorderBottomWidth(v) => self.layout_style.border.bottom = dimension(v),
            &BorderLeftWidth(v) => self.layout_style.border.left = dimension(v),

            // border_radius (px-only)
            &BorderTopLeftRadius(CssDimension::Px(v)) => self.render_style.border_radii[0] = v,
            &BorderTopRightRadius(CssDimension::Px(v)) => self.render_style.border_radii[1] = v,
            &BorderBottomRightRadius(CssDimension::Px(v)) => self.render_style.border_radii[2] = v,
            &BorderBottomLeftRadius(CssDimension::Px(v)) => self.render_style.border_radii[3] = v,

            // position
            // Position(v) => self.layout_style.position_type = position_type(v),
            // Top(v) => self.layout_style.position.top = dimension(v),
            // Right(v) => self.layout_style.position.right = dimension(v),
            // Bottom(v) => self.layout_style.position.bottom = dimension(v),
            // Left(v) => self.layout_style.position.left = dimension(v),

            // flex
            &FlexDirection(v) => self.layout_style.flex_direction = flex_direction(v),
            &FlexWrap(v) => self.layout_style.flex_wrap = flex_wrap(v),
            &FlexGrow(v) => self.layout_style.flex_grow = v,
            &FlexShrink(v) => self.layout_style.flex_shrink = v,
            &FlexBasis(v) => self.layout_style.flex_basis = dimension(v),
            &AlignContent(v) => self.layout_style.align_content = align(v),
            &AlignItems(v) => self.layout_style.align_items = align(v),
            &AlignSelf(v) => self.layout_style.align_self = align(v),
            &JustifyContent(v) => self.layout_style.justify_content = justify(v),

            // other
            &Display(v) => self.layout_style.display = display(v),

            _ => {}
        }
    }
}

fn display(value: CssDisplay) -> Display {
    match value {
        CssDisplay::None => Display::None,
        CssDisplay::Flex => Display::Flex,
        CssDisplay::Block => Display::Block,
        CssDisplay::Inline => Display::Inline,
        CssDisplay::InlineBlock => Display::InlineBlock,
        CssDisplay::Table => Display::Table,
        CssDisplay::TableRow => Display::TableRow,
        CssDisplay::TableCell => Display::TableCell,
        _ => Display::Block,
    }
}

fn flex_direction(value: CssFlexDirection) -> FlexDirection {
    match value {
        CssFlexDirection::Row => FlexDirection::Row,
        CssFlexDirection::Column => FlexDirection::Column,
        CssFlexDirection::RowReverse => todo!(),
        CssFlexDirection::ColumnReverse => todo!(),
    }
}

fn flex_wrap(value: CssFlexWrap) -> FlexWrap {
    match value {
        CssFlexWrap::NoWrap => FlexWrap::NoWrap,
        CssFlexWrap::Wrap => FlexWrap::Wrap,
        CssFlexWrap::WrapReverse => todo!(),
    }
}

fn dimension(value: CssDimension) -> Dimension {
    match value {
        CssDimension::Px(v) => Dimension::Px(v),
        CssDimension::Percent(v) => Dimension::Percent(v / 100.),
        CssDimension::Auto => Dimension::Auto,
        _ => todo!(),
    }
}

fn align(value: CssAlign) -> Align {
    match value {
        CssAlign::Auto => Align::Auto,
        CssAlign::FlexStart => Align::FlexStart,
        CssAlign::Center => Align::Center,
        CssAlign::FlexEnd => Align::FlexEnd,
        CssAlign::Stretch => Align::Stretch,
        CssAlign::Baseline => Align::Baseline,
        CssAlign::SpaceBetween => Align::SpaceBetween,
        CssAlign::SpaceAround => Align::SpaceAround,
    }
}

fn justify(value: CssJustify) -> Justify {
    match value {
        CssJustify::FlexStart => Justify::FlexStart,
        CssJustify::Center => Justify::Center,
        CssJustify::FlexEnd => Justify::FlexEnd,
        CssJustify::SpaceBetween => Justify::SpaceBetween,
        CssJustify::SpaceAround => Justify::SpaceAround,
        CssJustify::SpaceEvenly => Justify::SpaceEvenly,
    }
}
