use crate::css::{
    CssAlign, CssDimension, CssDisplay, CssFlexDirection, CssFlexWrap, CssJustify, CssPosition, CssStyleSheet,
    StyleProp, StyleResolver,
};
use crate::document::{Change, Document, NodeId, NodeKind};
use crate::gfx::{Canvas, GlBackend, PathCmd, RenderBackend, Text, TextStyle, Transform, Vec2, AABB, RGBA8};
use crate::layout::{
    Align, Dimension, Display, FlexDirection, FlexWrap, Justify, LayoutNodeId, LayoutResult, LayoutStyle, LayoutTree,
    Position, Size,
};
use crate::util::{BitSet, SlotMap};
use crate::windowing::Window;
use std::sync::Arc;

pub struct Renderer {
    document: Document,
    state: State,
    canvas: Canvas,
    window: Arc<Window>,
    backend: GlBackend,
}

#[derive(Default)]
struct State {
    layout_tree: LayoutTree,
    layout_nodes: SlotMap<NodeId, LayoutNodeId>,
    render_styles: SlotMap<NodeId, RenderStyle>,
    texts: SlotMap<NodeId, Text>,
}

impl Renderer {
    pub fn new(document: Document, win: &Window) -> Self {
        Self {
            document,
            canvas: Canvas::new(),
            state: State::default(),
            // TODO: make this whole fn unsafe?
            backend: unsafe { GlBackend::new(|s| win.get_proc_address(s) as _) },
            window: Window::find_by_id(win.id()).unwrap(),
        }
    }

    fn update(&mut self) {
        let changes = self.document.take_changes();
        let document = &self.document;
        let State {
            layout_tree,
            layout_nodes,
            render_styles,
            texts,
        } = &mut self.state;

        for ch in changes {
            match ch {
                Change::Created(id) => {
                    layout_nodes.put(id, layout_tree.create_node());
                }
                Change::Destroyed(id) => {
                    layout_tree.drop_node(layout_nodes[id]);
                    layout_nodes.remove(id);
                }
                Change::Changed(id) => {}
                Change::Inserted(id) => {
                    let parent = layout_nodes[document[id].parent_node().unwrap()];
                    let child = layout_nodes[id];
                    if let Some(next) = document[id].next_sibling() {
                        let before = layout_nodes[next];
                        layout_tree.insert_before(parent, child, before);
                    } else {
                        layout_tree.append_child(parent, child);
                    }
                }
                Change::Removed(id) => {
                    let parent = layout_nodes[document[id].parent_node().unwrap()];
                    let child = layout_nodes[id];
                    layout_tree.remove_child(parent, child);
                }
            }
        }
    }

    pub fn render(&mut self) {
        self.update();

        let mut ctx = RenderContext {
            canvas: &mut self.canvas,
        };

        // render_tree.visit(|item| ctx.render_item(item));

        unsafe { self.window.make_current() };
        self.backend.render_frame(ctx.canvas.flush());
        self.window.swap_buffers();
    }

    // pub fn client_rect(&self, ?) -> ? { self.xxx.with_render_state(|| ...) }

    pub fn resize(&mut self, width: f32, height: f32) {
        println!("TODO: Renderer::resize({}, {})", width, height);
    }
}

struct RenderContext<'a> {
    canvas: &'a mut Canvas,
    // + few context things like color, text-align, ?
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

    /*
    fn render_node(&mut self, parent_offset: Vec2, node: NodeId) {
        let render_node = &self.render_nodes[node];
        let layout_res = self.layout_tree.layout_result(render_node.layout_node);

        match render_node.dom_node.node_type() {
            NodeKind::Element | NodeKind::Document => self.render_container(
                parent_offset,
                &layout_res,
                render_node.render_style.as_ref().unwrap(),
                render_node.dom_node.child_nodes().into_iter(),
            ),
            NodeKind::Text => self.render_text(
                parent_offset,
                &layout_res,
                &render_node.dom_node.as_text().unwrap().data(),
            ),
        }
    }

    fn render_container(
        &mut self,
        parent_offset: Vec2,
        layout: &LayoutResult,
        style: &RenderStyle,
        children: impl Iterator<Item = NodeRef>,
    ) {
        if style.hidden {
            return;
        }

        let br = layout.border_rect();
        let border_rect = AABB::new(Vec2::new(br.left, br.top), Vec2::new(br.right, br.bottom));

        if style.opacity < 1. {
            // if style.opacity * current_opacity() < 0.01 {
            //     return
            // }

            // TODO: pop
            // push_opacity(opacity)
        }

        if let Some(transform) = style.transform {
            // TODO: pop
            // self.push_transform()
        }

        // for outline_shadow in ... {
        //     self.draw_outline_shadow(border_rect, outline_shadow);
        // }

        // if let Some(outline) = ... {
        //     self.draw_outline(border_rect, outline);
        // }

        // TODO: clip(padding_rect, border_radii) everything from here if overflow hidden/scroll?
        //       or maybe just children because everything accepts shape anyway?

        if style.bg_color[3] != 0 {
            self.draw_bg_color(border_rect, style.border_radii, style.bg_color);
        }

        // for image in ... {
        //     self.draw_image(border_rect, border_radii, image)
        // }

        // for inset_shadow in ... {
        //     // padding_rect
        //     self.draw_inset_shadow(padding_rect, border_radii, inset_shadow);
        // }

        // TODO: overflow: scroll

        for ch in children {
            self.render_node(parent_offset + border_rect.min, ch.id());
        }

        // if let Some(border) = ... {
        //     self.draw_border(border_rect, border_radii, border)
        // }
    }

    fn render_text(&mut self, parent_offset: Vec2, layout: &LayoutResult, text: &str) {}
    */
}

#[derive(Default)]
struct ResolvedStyle {
    layout_style: LayoutStyle,
    // text_style: TextStyle,
    render_style: RenderStyle,
}

struct RenderStyle {
    hidden: bool,
    transform: Option<Transform>,
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
            hidden: false,
            transform: None,
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
            &Width(v) => self.layout_style.size.width = dimension(v),
            &Height(v) => self.layout_style.size.height = dimension(v),
            &MinWidth(v) => self.layout_style.min_size.width = dimension(v),
            &MinHeight(v) => self.layout_style.min_size.height = dimension(v),
            &MaxWidth(v) => self.layout_style.max_size.width = dimension(v),
            &MaxHeight(v) => self.layout_style.max_size.height = dimension(v),

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
            &Position(v) => self.layout_style.position_type = position(v),
            &Top(v) => self.layout_style.position.top = dimension(v),
            &Right(v) => self.layout_style.position.right = dimension(v),
            &Bottom(v) => self.layout_style.position.bottom = dimension(v),
            &Left(v) => self.layout_style.position.left = dimension(v),

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
            &Display(v) => {
                self.render_style.hidden = v == CssDisplay::None;
                self.layout_style.display = display(v)
            }

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

fn position(value: CssPosition) -> Position {
    match value {
        CssPosition::Static => Position::Static,
        CssPosition::Relative => Position::Relative,
        CssPosition::Absolute => Position::Absolute,
        // TODO
        CssPosition::Sticky => Position::Static,
    }
}
