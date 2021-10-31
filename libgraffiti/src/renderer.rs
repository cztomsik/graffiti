use crate::css::{
    CssAlign, CssDimension, CssDisplay, CssFlexDirection, CssFlexWrap, CssJustify, CssStyleDeclaration, CssStyleSheet,
    StyleProp,
};
use crate::gfx::{Canvas, GlBackend, RenderBackend, Text, TextStyle, Vec2, AABB, RGBA8};
use crate::layout::{
    Align, Dimension, Display, FlexDirection, FlexWrap, Justify, LayoutBox, LayoutNode, LayoutStyle, Size,
};
use crate::{CharacterDataRef, DocumentRef, ElementRef, NodeRef, NodeType, Window};
use std::sync::Arc;

pub struct Renderer {
    document: DocumentRef,
    ua_sheet: CssStyleSheet,
    window: Arc<Window>,
    backend: GlBackend,
}

impl Renderer {
    pub fn new(document: DocumentRef, win: &Window) -> Self {
        // TODO: make this whole fn unsafe?
        let backend = unsafe { GlBackend::new(|s| win.get_proc_address(s) as _) };

        Self {
            window: Window::find_by_id(win.id()).unwrap(),
            document,
            ua_sheet: CssStyleSheet::default_ua_sheet(),
            backend,
        }
    }

    pub fn render(&self) {
        profile!();

        let layout_tree = self.create_layout_node(&self.document.as_node());
        let box_tree = layout_tree.calculate(Size {
            width: 1024.,
            height: 768.,
        });
        profile!("layout");

        let mut canvas = Canvas::new();
        let mut ctx = RenderContext { canvas: &mut canvas };

        ctx.render_box(0., 0., &box_tree);
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

    fn create_layout_node(&self, node: &NodeRef) -> LayoutNode {
        match node.node_type() {
            NodeType::Document => self.create_layout_node(&node.first_child().unwrap()),
            NodeType::Element => {
                let style = self.resolve_style(&node.downcast_ref::<ElementRef>().unwrap());

                LayoutNode::new(
                    style.layout_style,
                    node.child_nodes()
                        .iter()
                        .map(|ch| self.create_layout_node(ch))
                        .collect(),
                )
            }
            NodeType::Text => LayoutNode::new_text(Text::new(
                &node.downcast_ref::<CharacterDataRef>().unwrap().data(),
                &TextStyle::DEFAULT,
            )),
            _ => LayoutNode::new(LayoutStyle::default(), vec![]),
        }
    }

    fn resolve_style(&self, el: &ElementRef) -> ResolvedStyle {
        let mut resolved_style = ResolvedStyle::default();

        // match ua_sheet
        for rule in self.ua_sheet.rules() {
            if let Some(_spec) = rule.selector.match_element(el) {
                for p in rule.style().props().iter() {
                    resolved_style.apply_style_prop(p);
                }
            }
        }

        // TODO: match document.style_sheets()
        // for rule in self.style_sheets.flat_map(CssStyleSheet::rules) {
        //      if let Some(_spec) = rule.selector.match_element(el) { for p in rule.style().props().iter() { resolved_style.apply_style_prop(p) } }
        // }

        // append el.style()
        for p in el.style().props().iter() {
            resolved_style.apply_style_prop(p);
        }

        resolved_style
    }
}

struct RenderContext<'a> {
    canvas: &'a mut Canvas,
}

impl<'a> RenderContext<'a> {
    fn render_box(&mut self, x: f32, y: f32, layout_box: &LayoutBox) {
        let min = Vec2::new(x + layout_box.x(), y + layout_box.y());
        let max = min + Vec2::new(layout_box.width(), layout_box.height());
        let rect = AABB::new(min, max);

        if let Some(text) = &layout_box.text {
            // TODO: skip in layout?
            if layout_box.width() > 0. {
                self.canvas.fill_text(text, rect, [0, 0, 0, 255]);
            }
        } else {
            self.canvas.fill_rect(rect, [255, 0, 0, 30]);
        }

        for ch in layout_box.children() {
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
