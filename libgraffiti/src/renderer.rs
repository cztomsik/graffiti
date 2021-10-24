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
            backend,
        }
    }

    pub fn render(&self) {
        //let _sheet = CssStyleSheet::from(include_str!("../resources/ua.css"));

        let layout_tree = layout_node(&self.document.as_node());
        let box_tree = layout_tree.calculate(Size {
            width: 1024.,
            height: 768.,
        });

        let mut canvas = Canvas::new();
        let mut ctx = RenderContext { canvas: &mut canvas };

        ctx.render_box(0., 0., &box_tree);

        let frame = ctx.canvas.flush();
        unsafe { self.window.make_current() };
        self.backend.render_frame(frame);
        self.window.swap_buffers()
    }

    pub fn resize(&self, width: f32, height: f32) {
        println!("TODO: Renderer::resize({}, {})", width, height);
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

/*
fn style(style: &Style) -> RenderStyle {
    use super::css::*;

    let mut res = RenderStyle::DEFAULT;

    for p in style.props() {
        use StyleProp as P;

        match p {
            P::Display(CssDisplay::None) => res.hidden = true,
            P::BackgroundColor(c) => res.bg_color = Some([c.r, c.g, c.b, c.a]),
            P::OutlineColor(c) => {
                if let Some(o) = &mut res.outline {
                    o.1 = [c.r, c.g, c.b, c.a];
                } else {
                    res.outline = Some((0., [c.r, c.g, c.b, c.a]));
                }
            }
            P::OutlineWidth(CssDimension::Px(w)) => {
                if let Some(o) = &mut res.outline {
                    o.0 = *w;
                } else {
                    res.outline = Some((*w, [0, 0, 0, 0]));
                }
            }
            _ => {}
        }
    }

    res
}
*/

fn layout_node(node: &NodeRef) -> LayoutNode {
    match node.node_type() {
        NodeType::Document => layout_node(&node.first_child().unwrap()),
        NodeType::Element => LayoutNode::new(
            layout_style(&node.downcast_ref::<ElementRef>().unwrap().style()),
            node.child_nodes().iter().map(layout_node).collect(),
        ),
        NodeType::Text => LayoutNode::new_text(Text::new(
            &node.downcast_ref::<CharacterDataRef>().unwrap().data(),
            &TextStyle::DEFAULT,
        )),
        _ => LayoutNode::new(LayoutStyle::default(), vec![]),
    }
}

fn layout_style(style: &CssStyleDeclaration) -> LayoutStyle {
    let mut res = LayoutStyle::default();

    for p in style.props().iter() {
        use StyleProp::*;

        match p {
            // size
            &Width(v) => res.size.width = dimension(v),
            &Height(v) => res.size.height = dimension(v),
            &MinWidth(v) => res.min_size.width = dimension(v),
            &MinHeight(v) => res.min_size.height = dimension(v),
            &MaxWidth(v) => res.max_size.width = dimension(v),
            &MaxHeight(v) => res.max_size.height = dimension(v),

            // padding
            &PaddingTop(v) => res.padding.top = dimension(v),
            &PaddingRight(v) => res.padding.right = dimension(v),
            &PaddingBottom(v) => res.padding.bottom = dimension(v),
            &PaddingLeft(v) => res.padding.left = dimension(v),

            // margin
            &MarginTop(v) => res.margin.top = dimension(v),
            &MarginRight(v) => res.margin.right = dimension(v),
            &MarginBottom(v) => res.margin.bottom = dimension(v),
            &MarginLeft(v) => res.margin.left = dimension(v),

            // // position
            // Position(v) => res.position_type = position_type(v),
            // Top(v) => res.position.top = dimension(v),
            // Right(v) => res.position.right = dimension(v),
            // Bottom(v) => res.position.bottom = dimension(v),
            // Left(v) => res.position.left = dimension(v),

            // // flex
            &FlexDirection(v) => res.flex_direction = flex_direction(v),
            &FlexWrap(v) => res.flex_wrap = flex_wrap(v),
            &FlexGrow(v) => res.flex_grow = v,
            &FlexShrink(v) => res.flex_shrink = v,
            &FlexBasis(v) => res.flex_basis = dimension(v),
            &AlignContent(v) => res.align_content = align(v),
            &AlignItems(v) => res.align_items = align(v),
            &AlignSelf(v) => res.align_self = align(v),
            &JustifyContent(v) => res.justify_content = justify(v),

            // other
            &Display(v) => res.display = display(v),

            // TODO: remove
            _ => {}
        }
    }

    res
}

fn display(value: CssDisplay) -> Display {
    match value {
        CssDisplay::None => Display::None,
        CssDisplay::Flex => Display::Flex,
        CssDisplay::Block => Display::Block,
        CssDisplay::Inline => Display::Inline,
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
