use crate::commons::{ElementId, TextId, ElementChild, Pos, Bounds, Color};
use crate::box_layout::{BoxLayoutTree, BoxLayoutImpl};
use crate::text_layout::{GlyphInstance};

/// Goes through the tree and draws appropriate visuals at given bounds.
///
/// Drawing is expected to be done on the GPU which means we need to first
/// prepare the work before it can be executed on the `RenderBackend`.
///
/// The implementation is intentionally incorrect for the sake of simplicity and
/// CPU performance. It is designed to be fast enough to run on raspi and
/// it takes some shortcuts in order to do so. For example, it's possible to fool
/// clipping if you know how to.
///
/// This will be either fixed or maybe in the future, we might introduce some other
/// (optional) impl which would delegate to something more demanding but also
/// more complete (skia, maybe pathfinder but that would require GL3)
pub trait Renderer {
    fn realloc(&mut self, elements_count: ElementId, texts_count: TextId);

    fn set_border_radius(&mut self, element: ElementId, border_radius: Option<BorderRadius>);
    fn set_box_shadow(&mut self, element: ElementId, box_shadow: Option<BoxShadow>);
    fn set_background_color(&mut self, element: ElementId, color: Color);
    fn set_color(&mut self, element: ElementId, color: Color);
    fn set_image(&mut self, element: ElementId, image: Option<Image>);
    fn set_border(&mut self, element: ElementId, border: Option<Border>);

    fn set_text_glyphs<I>(&mut self, text: TextId, glyphs: I) where I: Iterator<Item=GlyphInstance>;

    // TODO: Index<ElementId/TextId, Output=Bounds> to break coupling
    // TODO: flags (bounds changed, tree changed)
    fn render(&mut self, element: ElementId, children: &[Vec<ElementChild>], box_layout: &BoxLayoutImpl);

    fn resize(&mut self, size: (f32, f32));
}

// some structs in the renderer-friendly way
// Option<BorderRadius> vs testing if any of the values is non-zero

#[derive(Debug, Clone, Copy)]
pub struct BorderRadius {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct BoxShadow {
    pub color: Color,
    pub offset: Pos,
    pub blur: f32,
    pub spread: f32,
}

// TODO
pub struct Image {}
pub struct Border {}

// impl starts here

pub struct RendererImpl {
    backend: GlRenderBackend,
    ui_state: UiState,

    // reused to avoid allocs
    render_rects: Vec<RectId>,
    render_rects_bounds: Vec<Bounds>,
    render_ops: Vec<RenderOp>,
}

// TODO: rename
// (read-only) input for rendering which was prepared during set_* calls
struct UiState {
    //flags: Vec<FastPathFlags>,
    colors: Vec<Color>,
    //border_radii: BTreeMap<ElementId, BorderRadius>,
    //box_shadows: BTreeMap<ElementId, BoxShadow>,
    //images: BTreeMap<ElementId, Image>,
    //borders: BTreeMap<Element, Border>,
}

type FastPathFlags = u8;

impl RendererImpl {
    pub fn new(size: (f32, f32)) -> Self {
        let mut res = RendererImpl {
            backend: GlRenderBackend::new(),
            ui_state: UiState {
                colors: Vec::new(),
            },

            render_rects: Vec::new(),
            render_rects_bounds: Vec::new(),
            render_ops: Vec::new()
        };

        res.resize(size);

        res
    }
}

impl Renderer for RendererImpl {
    fn realloc(&mut self, elements_count: ElementId, texts_count: TextId) {
        self.ui_state.colors.resize(elements_count, Color::BLACK);
        self.render_rects_bounds.resize(elements_count, Bounds::ZERO);

        self.backend.realloc(elements_count, texts_count);
    }

    fn set_border_radius(&mut self, _element: ElementId, _border_radius: Option<BorderRadius>) {
        todo!()
    }

    fn set_box_shadow(&mut self, _element: ElementId, _shadow: Option<BoxShadow>) {
        todo!()
    }

    fn set_background_color(&mut self, element: ElementId, color: Color) {
        self.backend.set_rect_color(element as RectId, color);
    }

    fn set_color(&mut self, element: ElementId, color: Color) {
        self.ui_state.colors[element] = color
    }

    fn set_image(&mut self, _element: ElementId, _image: Option<Image>) {
        todo!()
    }

    fn set_border(&mut self, _element: ElementId, _border: Option<Border>) {
        todo!()
    }

    fn set_text_glyphs<I>(&mut self, text: TextId, glyphs: I) where I: Iterator<Item=GlyphInstance> {
        self.backend.set_text_glyphs(text, glyphs);
    }

    fn render(&mut self, element: ElementId, children: &[Vec<ElementChild>], box_layout: &BoxLayoutImpl) {
        unsafe {
            self.render_rects.set_len(0);
            self.render_ops.set_len(0);
        }

        // TODO:
        // - skip bounds
        // - skip rects
        // - skip ops
        let mut ctx = RenderContext {
            children,
            box_layout,

            ui_state: &self.ui_state,
            parent_bounds: Bounds::ZERO,

            rects: &mut self.render_rects,
            rects_bounds: &mut self.render_rects_bounds,
            ops: &mut self.render_ops,
        };

        ctx.render_element(element);

        // TODO: set all at once
        for (i, b) in self.render_rects_bounds.iter().enumerate() {
            self.backend.set_rect_bounds(i, *b);
        }

        self.backend.render(&self.render_rects, &self.render_ops);
    }

    fn resize(&mut self, size: (f32, f32)) {
        self.backend.resize(size);
    }
}

struct RenderContext<'a> {
    children: &'a [Vec<ElementChild>],
    box_layout: &'a BoxLayoutImpl,

    ui_state: &'a UiState,
    parent_bounds: Bounds,

    rects: &'a mut Vec<RectId>,
    rects_bounds: &'a mut Vec<Bounds>,
    ops: &'a mut Vec<RenderOp>,
}

impl <'a> RenderContext<'a> {
    fn render_element(&mut self, element: ElementId) {
        // TODO: skip if empty/hidden

        // TODO: skip if layout is up-to-date
        let bounds = self.box_layout.get_element_bounds(element).relative_to(self.parent_bounds.a);
        self.rects_bounds[element] = bounds;

        // TODO: clip

        // TODO: shadow

        self.rects.push(element as RectId);

        // TODO: skip if transparent
        // TODO: batching & interleaving
        self.ops.push(RenderOp::DrawRects { count: 1 });

        // TODO: bg color
        // TODO: gradient
        // TODO: image
        // TODO: inner shadow

        // TODO: clip children too if overflow: hidden
        // render children
        let parent_bounds = self.parent_bounds;
        self.parent_bounds = bounds;

        for c in &self.children[element] {
            match c {
                // TODO: replace recursion with iteration
                ElementChild::Element { id } => self.render_element(*id),
                ElementChild::Text { id } => self.render_text(*id, self.box_layout.get_text_bounds(*id).a, self.ui_state.colors[element]),
            }
        }

        // TODO: border

        self.parent_bounds = parent_bounds;
    }

    fn render_text(&mut self, id: TextId, pos: Pos, color: Color) {
        self.ops.push(RenderOp::DrawText { id, pos: pos.relative_to(self.parent_bounds.a), color, distance_factor: 1.1428571429 });
    }
}

/// Low-level renderer, specific to the given graphics api (OpenGL/Vulkan/SW)
/// Knows how to draw primitives, prepared by higher-level `Renderer`
///
/// Backend does the real drawing but it's also very simple and can't do any
/// optimizations and has absolutely no idea about scene or anything else.
/// You don't want to use it directly and so it's useless just by itself.
pub trait RenderBackend {
    fn realloc(&mut self, rects_count: RectId, texts_count: TextId);

    fn set_rect_bounds(&mut self, rect: RectId, bounds: Bounds);
    fn set_rect_color(&mut self, rect: RectId, color: Color);

    fn set_text_glyphs<I>(&mut self, text: TextId, glyphs: I) where I: Iterator<Item=GlyphInstance>;

    // TODO: initial_clip (update region) to reduce overdraw (of up-to-date parts)
    // and given how GPUs work it should finish faster (more free units) 
    fn render(&mut self, rects: &[RectId], ops: &[RenderOp]);

    fn resize(&mut self, size: (f32, f32));
}

pub type RectId = usize;

/// Primitives/ops implemented by backend.
#[derive(Debug)]
pub enum RenderOp {
    // TODO: opacity, translate, scale, clip, clipRadii
    DrawRects { count: usize },
    DrawText { id: TextId, color: Color, pos: Pos, distance_factor: f32 },
    // TODO: text, image, border, shadow, ...
}

mod gl;
use self::gl::GlRenderBackend;







/*


impl <'a> RenderContext<'a> {
    // TODO: create_text() -> TextId & Batch::Text(text_id)
    fn draw_text(&mut self, text: &Text, color: Color, glyphs: impl Iterator<Item = GlyphInstance>) {
        // TODO: should be uniform
        let origin = self.bounds.a;

        silly!("text {:?} {:?}", &origin, &text.text);

        self.builder.frame.batches.push(Batch::AlphaRects { num: self.builder.count });
        self.builder.append_indices();
        self.builder.count = 0;

        for GlyphInstance { bounds, coords } in glyphs {
            self.builder.push_quad(false, &Quad::new(bounds.relative_to(origin), [
                coords.a,
                Pos::new(coords.b.x, coords.a.y),
                Pos::new(coords.a.x, coords.b.y),
                coords.b,
            ]));
        }

        // TODO: read from font file
        let texture_font_size = 42.;
        let px_range = 3.;
        // https://github.com/Chlumsky/msdfgen/issues/22
        // https://github.com/Chlumsky/msdfgen/issues/36
        let distance_factor = (text.font_size / texture_font_size) * px_range;

        self.builder.frame.batches.push(Batch::Text { color, distance_factor, num: self.builder.count });
        self.builder.append_indices();
        self.builder.count = 0;
    }
}

*/