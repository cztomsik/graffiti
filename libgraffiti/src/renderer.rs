#![allow(unused)]

use crate::css::{
    CssAlign, CssDimension, CssDisplay, CssFlexDirection, CssFlexWrap, CssJustify, CssStyleSheet, StyleProp,
    StyleResolver,
};
use crate::gfx::{Canvas, GlBackend, RenderBackend, Text, TextStyle, Vec2, AABB, RGBA8};
use crate::layout::{
    Align, Dimension, Display, FlexDirection, FlexWrap, Justify, LayoutNodeId, LayoutStyle, LayoutTree, Size,
};
use crate::util::{BitSet, Edge, SlotMap};
use crate::{CharacterDataRef, DocumentRef, DomEvent, ElementRef, NodeId, NodeRef, NodeType, Window};
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
    layout_nodes: SlotMap<NodeId, LayoutNodeId>,
    dirty_nodes: BitSet,
}

impl Renderer {
    pub fn new(document: DocumentRef, win: &Window) -> Self {
        // TODO: make this whole fn unsafe?
        let backend = unsafe { GlBackend::new(|s| win.get_proc_address(s) as _) };

        let state = Rc::new(RefCell::new(State::default()));
        let listener = Self::create_listener(Rc::clone(&state));

        // connect
        document.add_listener(Rc::clone(&listener));
        for node in document.all_nodes() {
            listener(&DomEvent::NodeCreated(&node));
        }
        let mut parent = document.id();
        for edge in document.traverse().iter().skip(1) {
            match *edge {
                Edge::Start(id) => {
                    listener(&DomEvent::AppendChild(
                        &document.find_node(parent).unwrap(),
                        &document.find_node(id).unwrap(),
                    ));
                    parent = id;
                }
                Edge::End(id) => parent = id,
            }
        }
        let root_layout_node = state.borrow().layout_nodes[document.id()];
        state.borrow_mut().layout_tree.set_style(
            root_layout_node,
            LayoutStyle {
                display: Display::Block,
                ..Default::default()
            },
        );

        Self {
            window: Window::find_by_id(win.id()).unwrap(),
            document,
            style_resolver: StyleResolver::new(vec![Rc::new(CssStyleSheet::default_ua_sheet())]),
            state,
            listener,
            backend,
        }
    }

    fn create_listener(state: Rc<RefCell<State>>) -> Rc<dyn Fn(&DomEvent)> {
        Rc::new(move |event| {
            let State {
                layout_tree,
                layout_nodes,
                dirty_nodes,
            } = &mut *state.borrow_mut();
            match event {
                DomEvent::NodeCreated(node) => {
                    layout_nodes.put(node.id(), layout_tree.create_node());
                    dirty_nodes.grow(node.id());
                }
                &DomEvent::NodeDestroyed(id) => {
                    layout_tree.drop_node(layout_nodes.remove(id).unwrap());
                    // if whole subtree gets freed, it might not be removed at all
                    dirty_nodes.remove(id);
                }
                DomEvent::AppendChild(parent, child) => {
                    layout_tree.append_child(layout_nodes[parent.id()], layout_nodes[child.id()]);
                    dirty_nodes.add(child.id());
                    // TODO: descendants should be dirty too
                }
                DomEvent::InsertBefore(parent, child, before) => {
                    layout_tree.insert_before(
                        layout_nodes[parent.id()],
                        layout_nodes[child.id()],
                        layout_nodes[before.id()],
                    );
                    dirty_nodes.add(child.id());
                }
                DomEvent::RemoveChild(parent, child) => {
                    layout_tree.append_child(layout_nodes[parent.id()], layout_nodes[child.id()]);
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
            layout_tree: &state.layout_tree,
        };

        profile!();
        ctx.render_box(0., 0., state.layout_nodes[self.document.id()]);
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
            layout_nodes,
            dirty_nodes,
        } = &mut *self.state.borrow_mut();

        for id in dirty_nodes.iter() {
            let node = self.document.find_node(id).unwrap();
            if let Some(el) = node.as_element() {
                println!("update style {:?}", (el.local_name(), id));

                let mut res = self.style_resolver.resolve_style(&el, ResolvedStyle::apply_style_prop);
                for p in el.style().props().iter() {
                    res.apply_style_prop(p);
                }

                layout_tree.set_style(layout_nodes[id], res.layout_style);
            } else if let Some(cdata) = node.as_character_data() {
                println!("TODO: update text/comment");
            }
        }
        dirty_nodes.clear();
        profile!("css");

        layout_tree.calculate(layout_nodes[self.document.id()], 1024., 768.);
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
    layout_tree: &'a LayoutTree,
}

impl<'a> RenderContext<'a> {
    fn render_box(&mut self, x: f32, y: f32, layout_node: LayoutNodeId) {
        let res = self.layout_tree.layout_result(layout_node);

        let min = Vec2::new(x + res.x(), y + res.y());
        let max = min + Vec2::new(res.outer_width(), res.outer_height());
        let rect = AABB::new(min, max);

        // if let Some(text) = &layout_box.text {
        //     // TODO: skip in layout?
        //     if layout_box.width() > 0. {
        //         self.canvas.fill_text(text, rect, [0, 0, 0, 255]);
        //     }
        // } else {
        self.canvas.fill_rect(rect, [255, 0, 0, 30]);
        // }

        for ch in self.layout_tree.children(layout_node) {
            self.render_box(min.x, min.y, ch);
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

#[derive(Default)]
struct ResolvedStyle {
    layout_style: LayoutStyle,
    render_style: RenderStyle,
}

#[derive(Default)]
struct RenderStyle {
    bg_color: RGBA8,
}

impl ResolvedStyle {
    fn apply_style_prop(&mut self, prop: &StyleProp) {
        use StyleProp::*;
        match prop {
            // size
            &Width(v) => self.layout_style.width = dimension(v),
            &Height(v) => self.layout_style.height = dimension(v),
            &MinWidth(v) => self.layout_style.min_width = dimension(v),
            &MinHeight(v) => self.layout_style.min_height = dimension(v),
            &MaxWidth(v) => self.layout_style.max_width = dimension(v),
            &MaxHeight(v) => self.layout_style.max_height = dimension(v),

            // padding
            &PaddingTop(v) => self.layout_style.padding.top = dimension(v),
            &PaddingRight(v) => self.layout_style.padding.right = dimension(v),
            &PaddingBottom(v) => self.layout_style.padding.bottom = dimension(v),
            &PaddingLeft(v) => self.layout_style.padding.left = dimension(v),

            // margin
            &MarginTop(v) => self.layout_style.margin.top = dimension(v),
            &MarginRight(v) => self.layout_style.margin.right = dimension(v),
            &MarginBottom(v) => self.layout_style.margin.bottom = dimension(v),
            &MarginLeft(v) => self.layout_style.margin.left = dimension(v),

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
            &BackgroundColor(v) => self.render_style.bg_color = [v.r, v.g, v.b, v.a],
            // TODO: remove
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
