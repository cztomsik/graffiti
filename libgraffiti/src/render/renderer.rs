use super::backend::{DrawCall, Frame, Quad, Vertex, RGBA8};
use crate::{Document, NodeId, NodeType, Rect};
use once_cell::sync::Lazy;
use owned_ttf_parser::{AsFaceRef, OwnedFace};

// not checked/enforced for now but debug_assert! might be enough
pub struct AABB {
    a: (f32, f32),
    b: (f32, f32),
}

static SANS_SERIF_FACE: Lazy<Font> = Lazy::new(|| {
    use fontdb::{Database, Family, Query};

    let mut db = Database::new();
    db.set_sans_serif_family("Arial");
    db.load_system_fonts();

    let id = db
        .query(&Query {
            families: &[Family::SansSerif],
            ..Default::default()
        })
        .expect("no default font");

    let face = db
        .with_face_data(id, |data, i| OwnedFace::from_vec(data.to_owned(), i).unwrap())
        .unwrap();
    let scale = 16. / face.as_face_ref().units_per_em().unwrap() as f32;

    Font { face, scale }
});

struct Font {
    face: OwnedFace,
    scale: f32,
}

pub struct Renderer {
    // TODO: fonts, images, caching, ...
}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render<'a>(
        &mut self,
        document: &'a Document,
        layout_rect: &'a dyn Fn(NodeId) -> Rect,
    ) -> Frame {
        let root = document.root();

        let mut ctx = RenderContext {
            document,
            frame: Frame::new(),
            layout_rect,
        };

        ctx.render_node((0., 0.), root);
        
        ctx.frame
    }
}

struct RenderContext<'a> {
    document: &'a Document,
    frame: Frame,
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
            self.render_element(rect, /*self.resolved_styles.lookup(node),*/ self.document.children(node));
        }

        if self.document.node_type(node) == NodeType::Text {
            self.render_text_node(rect, self.document.cdata(node));
        }
    }

    fn render_element(&mut self, rect: Rect, /*style: &ResolvedStyle,*/ children: impl Iterator<Item = NodeId>) {
        //let _s = style;

        self.fill_rect(rect, [255, 63, 63, 100]);

        //if let Some(color) = s.background_color {}

        //if let Some(xxx) = x.xxx {}

        for ch in children {
            self.render_node(rect.pos, ch);
        }
    }

    fn render_text_node(&mut self, rect: Rect, text: &str) {
        let mut pos = rect.pos;

        let scale = SANS_SERIF_FACE.scale;
        let face = SANS_SERIF_FACE.face.as_face_ref();

        for c in text.chars() {
            if let Some(glyph_id) = face.glyph_index(c) {
                if let Some(glyph_rect) = face.glyph_bounding_box(glyph_id) {
                    self.fill_rect(
                        Rect {
                            pos,
                            size: (glyph_rect.width() as f32 * scale, glyph_rect.height() as f32 * scale),
                        },
                        [0, 0, 0, 200],
                    );
                    pos.0 += face.glyph_hor_advance(glyph_id).unwrap_or(0) as f32 * scale;
                }
            }
        }
    }

    fn fill_rect(&mut self, rect: Rect, color: RGBA8 /*, fill: Fill*/) {
        let Rect { pos, size } = rect;

        // TODO
        let z = 1.;
        let uv = [0., 0.];

        self.frame.quads.push(Quad {
            vertices: [
                Vertex {
                    xyz: [pos.0, pos.1, z],
                    uv,
                    color,
                },
                Vertex {
                    xyz: [pos.0 + size.0, pos.1, z],
                    uv,
                    color,
                },
                Vertex {
                    xyz: [pos.0, pos.1 + size.1, z],
                    uv,
                    color,
                },
                Vertex {
                    xyz: [pos.0 + size.0, pos.1 + size.1, z],
                    uv,
                    color,
                },
            ],
        });
        self.frame.draw_calls.push(DrawCall { len: 1 });
    }
}
