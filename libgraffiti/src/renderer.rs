use crate::gfx::{Canvas, Frame, Text, TextStyle, Vec2, AABB};
use crate::{Document, NodeId, NodeType};

pub struct Renderer {
    canvas: Canvas,
}

impl Renderer {
    pub fn new() -> Self {
        Self { canvas: Canvas::new() }
    }

    pub fn render(&mut self, document: &Document, layout_fn: &dyn Fn(NodeId) -> AABB) -> Frame {
        let root = document.root();

        let mut ctx = RenderContext {
            document,
            canvas: &mut self.canvas,
            layout_fn,
        };

        ctx.render_node(Vec2::ZERO, root);

        ctx.canvas.flush()
    }
}

struct RenderContext<'a> {
    document: &'a Document,
    canvas: &'a mut Canvas,
    layout_fn: &'a dyn Fn(NodeId) -> AABB,
}

impl<'a> RenderContext<'a> {
    fn render_node(&mut self, offset: Vec2, node: NodeId) {
        let rect = (self.layout_fn)(node) + offset;

        if self.document.node_type(node) == NodeType::Document {
            for ch in self.document.children(node) {
                self.render_node(rect.min, ch)
            }
        }

        if self.document.node_type(node) == NodeType::Element {
            self.render_element(rect, self.document.child_nodes(node));
        }

        if self.document.node_type(node) == NodeType::Text {
            self.render_text_node(rect, self.document.cdata(node));
        }
    }

    fn render_element(&mut self, rect: AABB, /*style: &ResolvedStyle,*/ children: impl Iterator<Item = NodeId>) {
        self.canvas.set_fill_color([255, 63, 63, 32]);
        self.canvas.fill_rect(rect);

        for ch in children {
            self.render_node(rect.min, ch);
        }
    }

    fn render_text_node(&mut self, rect: AABB, text: &str) {
        let text = Text::new(text, &TextStyle::DEFAULT);

        self.canvas.set_fill_color([0, 0, 0, 255]);
        self.canvas.fill_text(&text, rect);
    }
}
