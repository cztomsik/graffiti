// TODO: display: none
// TODO: transform: scale()
// TODO: opacity
// TODO: scroll
// TODO: text nodes

use std::collections::BTreeMap;
use crate::commons::{Pos, Bounds, SurfaceId, Color};
use crate::util::{Storage};
use crate::text_layout::{TextLayout, Text};

/// Goes through the tree and draws appropriate visuals using positions & sizes from
/// the `box_layout` & `text_layout` respectively.
///
/// Drawing is expected to be done on the GPU which means we need to first
/// prepare the work (`Frame`) before it can be executed on the `Backend`.
///
/// Some of the work can be shared between the frames so that's why this is
/// stateful and why it's necessary to call some methods if the scene
/// has been changed.
///
/// This implementation is intentionally incorrect for the sake of simplicity and
/// CPU performance. It is designed to be fast enough to run on raspi and
/// it takes some shortcuts in order to do so. For example, it's possible to fool
/// clipping if you know how to.
///
/// This will be either fixed or maybe in the future, we might introduce some other
/// (optional) impl which would delegate to something more demanding but also
/// more complete (skia, maybe pathfinder but that would require GL3)
pub struct Renderer {
    backend: RenderBackend,
    pub children: Vec<Vec<SurfaceId>>,
    texts: BTreeMap<SurfaceId, f32>,
}

impl Renderer {
    pub fn new(width: i32, height: i32) -> Self {
        let mut res = Renderer {
            backend: RenderBackend::new(),
            children: Vec::new(),
            texts: BTreeMap::new(),
        };

        res.resize(width, height);
        res.alloc();

        res
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.backend.resize(width, height);
    }

    pub fn alloc(&mut self) {
        self.children.push(Vec::new());
        self.backend.create_rect();
    }

    pub fn insert_at(&mut self, parent: SurfaceId, child: SurfaceId, index: usize) {
        self.children[parent].insert(index, child);
    }

    pub fn remove_child(&mut self, parent: SurfaceId, child: SurfaceId) {
        self.children[parent].retain(|ch| *ch != child);
    }

    pub fn render(&mut self, all_bounds: &[Bounds], _text_layout: &TextLayout) {
        let mut frame = Frame {
            rects: Vec::new(),
            ops: vec![]
        };

        self.draw_surface(0, all_bounds, Bounds::zero(), &mut frame);

        // TODO: interleaving!!!!
        frame.ops.push(RenderOp::DrawRects { count: frame.rects.len() });

        self.backend.render_frame(frame)
    }

    pub fn set_text_color(&mut self, _surface: SurfaceId, _color: Color) {}

    pub fn set_background_color(&mut self, surface: SurfaceId, color: Option<Color>) {
        self.backend.set_rect_color(surface, color.unwrap_or(Color::TRANSPARENT));
    }

    // TODO: cache glyphs, after box_layout
    pub fn set_text(&mut self, surface: SurfaceId, text: Option<Text>) {
        self.texts.set(surface, text.map(|t| t.font_size));
    }

    fn draw_surface(&mut self, surface: SurfaceId, all_bounds: &[Bounds], parent_bounds: Bounds, frame: &mut Frame) {
        let bounds = all_bounds[surface].relative_to(parent_bounds.a);

        self.backend.set_rect_bounds(surface, bounds);

        frame.rects.push(surface);

        for c in self.children[surface].clone() {
            self.draw_surface(c, all_bounds, bounds, frame);
        }
    }
}

mod backend;
use backend::{RenderBackend};

/// The work which has to be done to render one "frame".
/// It is split to multiple batches/ops, sometimes because it's faster
/// and sometimes because of texture/buffer limits, correct drawing-order, etc.
#[derive(Debug)]
pub struct Frame {
    rects: Vec<RectId>,
    ops: Vec<RenderOp>
}

/// Primitives/ops in the GPU pipeline implemented by backend.
#[derive(Debug)]
pub enum RenderOp {
    // SetAlpha
    // SetOpacity
    // SetTranslate
    // SetScale
    // SetClip { bounds, radii }
    DrawRects { count: usize },
    //DrawText { text: TextId },
    //DrawImage { image: ImageId },
    //DrawBorder
}

pub type RectId = usize;
pub type TextId = usize;







#[derive(Debug, Clone, Copy)]
pub struct BoxShadow {
    pub color: Color,
    pub offset: Pos,
    pub blur: f32,
    pub spread: f32,
}








/*

use crate::commons::{Pos, Bounds, SurfaceId, Color};
use std::collections::BTreeMap;
use std::io::Write;
use crate::text_layout::{TextLayout, Text, GlyphInstance};
use crate::util::Storage;


impl Renderer {
    // TODO: Index<SurfaceId, Iterator<GlyphInstance>> could be enough
    pub fn render(&mut self, all_bounds: &[Bounds], text_layout: &TextLayout) {
        let frame = self.prepare_frame(all_bounds, text_layout);
        self.backend.render_frame(frame);
    }

    pub fn set_text_color(&mut self, surface: SurfaceId, color: Color) {
        self.scene.text_colors[surface] = color;
    }

    pub fn set_text(&mut self, surface: SurfaceId, text: Option<Text>) {
        // TODO: inspect perf, create/remove_text, cache buffers
        self.scene.texts.set(surface, text);
    }

    fn prepare_frame(&mut self, all_bounds: &[Bounds], text_layout: &TextLayout) -> Frame {
        let root = 0;
        let mut builder = FrameBuilder::new();

        let mut context = RenderContext {
            text_layout,

            scene: &self.scene,
            all_bounds,
            bounds: all_bounds[root],

            // TODO: find a way to avoid allocations (maybe pool frames?)
            builder: &mut builder,
        };

        context.draw_surface(root);
        builder.finish();

        builder.frame
    }
}

struct RenderContext<'a> {
    text_layout: &'a TextLayout,
    scene: &'a Scene,
    all_bounds: &'a[Bounds],

    // TODO: clip
    bounds: Bounds,

    builder: &'a mut FrameBuilder
}

impl <'a> RenderContext<'a> {
    // TODO: bitflags
    fn draw_surface(&mut self, id: SurfaceId) {
        let parent_bounds = self.bounds;

        // TODO: maybe layout should do this too and provide bounds in absolute coords
        self.bounds = self.all_bounds[id].relative_to(parent_bounds.a);

        if let Some(text) = self.scene.texts.get(&id) {
            self.draw_text(text, self.scene.text_colors[id], self.text_layout.get_glyphs(id));
        }

        // TODO: try to avoid recursion?
        for c in &self.scene.children[id] {
            self.draw_surface(*c);
        }

        // restore because of recursion
        self.bounds = parent_bounds;
    }

    fn draw_background_color(&mut self, color: &Color) {
        self.builder.push_rect(self.bounds, color);
    }

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

// some structs in the renderer-friendly way
// for example border radius is just a bunch of numbers but it means we will
// need to do clipping and so it's together and it's either there or not at all

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

*/