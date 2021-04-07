use crate::gfx::{Canvas, Frame, Text, TextStyle};
use crate::{Document, NodeId, NodeType, Rect};

pub struct Renderer {
    canvas: Canvas,
}

impl Renderer {
    pub fn new() -> Self {
        Self { canvas: Canvas::new() }
    }

    pub fn render<'a>(&mut self, document: &'a Document, layout_rect: &'a dyn Fn(NodeId) -> Rect) -> Frame {
        let root = document.root();

        let mut ctx = RenderContext {
            document,
            canvas: &mut self.canvas,
            layout_rect,
        };

        ctx.render_node((0., 0.), root);

        ctx.canvas.flush()
    }
}

struct RenderContext<'a> {
    document: &'a Document,
    canvas: &'a mut Canvas,
    layout_rect: &'a dyn Fn(NodeId) -> Rect,
}

impl<'a> RenderContext<'a> {
    fn render_node(&mut self, offset: (f32, f32), node: NodeId) {
        let mut rect = (self.layout_rect)(node);

        rect.pos.0 += offset.0;
        rect.pos.1 += offset.1;

        if self.document.node_type(node) == NodeType::Document {
            for ch in self.document.children(node) {
                self.render_node(rect.pos, ch)
            }
        }

        if self.document.node_type(node) == NodeType::Element {
            self.render_element(rect, self.document.child_nodes(node));
        }

        if self.document.node_type(node) == NodeType::Text {
            self.render_text_node(rect, self.document.cdata(node));
        }
    }

    fn render_element(&mut self, rect: Rect, /*style: &ResolvedStyle,*/ children: impl Iterator<Item = NodeId>) {
        let Rect { pos, size } = rect;

        self.canvas.set_fill_color([255, 63, 63, 32]);
        self.canvas.fill_rect(pos.into(), size.into());

        for ch in children {
            self.render_node(rect.pos, ch);
        }
    }

    fn render_text_node(&mut self, rect: Rect, text: &str) {
        //let text_style = TextStyle::DEFAULT;
        let text = Text::new(text);

        self.canvas.set_fill_color([0, 0, 0, 200]);
        self.canvas.fill_text(&text, rect.pos.into());
    }
}
