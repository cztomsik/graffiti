use crate::css::Style;
use crate::gfx::{Canvas, Frame, Text, Vec2, AABB, RGBA8};
use crate::layout::LayoutNode;
use crate::util::SlotMap;
use crate::{Document, NodeId, NodeType};
use std::cell::RefCell;
use std::rc::Rc;

pub struct RenderStyle {
    pub hidden: bool,
    pub bg_color: Option<RGBA8>,
    pub outline: Option<(f32, RGBA8)>,
}

impl RenderStyle {
    pub const DEFAULT: Self = Self {
        hidden: false,
        bg_color: None,
        outline: None,
    };
}

pub struct Renderer {
    document: Rc<RefCell<Document>>,
    layout_nodes: Rc<RefCell<SlotMap<NodeId, LayoutNode>>>,
    styles: Rc<RefCell<SlotMap<NodeId, Style>>>,
    texts: Rc<RefCell<SlotMap<NodeId, Text>>>,
    canvas: Rc<RefCell<Canvas>>,
}

impl Renderer {
    pub fn new(
        document: &Rc<RefCell<Document>>,
        layout_nodes: &Rc<RefCell<SlotMap<NodeId, LayoutNode>>>,
        styles: &Rc<RefCell<SlotMap<NodeId, Style>>>,
        texts: &Rc<RefCell<SlotMap<NodeId, Text>>>,
    ) -> Self {
        Self {
            document: Rc::clone(&document),
            layout_nodes: Rc::clone(&layout_nodes),
            styles: Rc::clone(&styles),
            texts: Rc::clone(&texts),
            canvas: Rc::new(RefCell::new(Canvas::new())),
        }
    }

    pub fn render<'a>(&'a self) -> Frame {
        let document = &*self.document.borrow();
        let layout_nodes = &*self.layout_nodes.borrow();
        let styles = &*self.styles.borrow();
        let texts = &*self.texts.borrow();
        let root = document.root();

        let mut ctx = RenderContext {
            document,
            canvas: &mut self.canvas.borrow_mut(),
            layout_nodes,
            styles,
            texts,
        };

        ctx.render_node(Vec2::ZERO, root);

        ctx.canvas.flush()
    }
}

struct RenderContext<'a> {
    document: &'a Document,
    layout_nodes: &'a SlotMap<NodeId, LayoutNode>,
    styles: &'a SlotMap<NodeId, Style>,
    texts: &'a SlotMap<NodeId, Text>,
    canvas: &'a mut Canvas,
}

impl<'a> RenderContext<'a> {
    fn render_node(&mut self, offset: Vec2, node: NodeId) {
        let ln = &self.layout_nodes[node];
        let min = offset + ln.offset().into();
        let max = min + ln.size().into();
        let rect = AABB::new(min, max);

        if self.document.node_type(node) == NodeType::Document {
            for ch in self.document.children(node) {
                self.render_node(rect.min, ch)
            }
        }

        if self.document.node_type(node) == NodeType::Element {
            self.render_element(rect, &style(&self.styles[node]), self.document.child_nodes(node));
        }

        if self.document.node_type(node) == NodeType::Text {
            // TODO: color
            self.canvas.fill_text(&self.texts[node], rect, [0, 0, 0, 255]);
        }
    }

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
}

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
