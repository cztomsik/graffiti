// super-simple GPU-first 2D graphics
// x outputs (textured) vertices + draw "ops"
// x easy to integrate (and to compose)

use super::{CachedGlyph, GlyphCache, GlyphPos, Text, Vec2};

pub type RGBA8 = [u8; 4];

pub struct Canvas {
    states: Vec<State>,
    frame: Frame,
    glyph_cache: GlyphCache,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            states: vec![State::DEFAULT],
            frame: Frame::new(),
            glyph_cache: GlyphCache::new(),
        }
    }

    // TODO: channel?
    pub fn flush(&mut self) -> Frame {
        // TODO: CoW or something...
        self.frame
            .draw_ops
            .insert(0, DrawOp::TexData(self.glyph_cache.tex_data().clone()));

        std::mem::replace(&mut self.frame, Frame::new())
    }

    pub fn save(&mut self) {
        self.states.push(self.state().clone());
    }

    pub fn restore(&mut self) {
        if self.states.len() > 1 {
            drop(self.states.pop())
        } else {
            self.states[0] = State::DEFAULT
        }
    }

    fn state(&self) -> &State {
        self.states.last().unwrap()
    }

    fn state_mut(&mut self) -> &mut State {
        self.states.last_mut().unwrap()
    }

    pub fn opacity(&self) -> f32 {
        self.state().opacity
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.state_mut().opacity = opacity;
    }

    pub fn fill_color(&self) -> RGBA8 {
        self.state().fill_color
    }

    pub fn set_fill_color(&mut self, fill_color: RGBA8) {
        self.state_mut().fill_color = fill_color;
    }

    pub fn fill_rect(&mut self, pos: Vec2, size: Vec2) {
        let color = self.state().fill_color;
        let uv = Vec2::ZERO;

        self.frame.vertices.extend_from_slice(&[
            Vertex::new(pos, uv, color),
            Vertex::new(Vec2::new(pos.x + size.x, pos.y), uv, color),
            Vertex::new(Vec2::new(pos.x, pos.y + size.y), uv, color),
            Vertex::new(Vec2::new(pos.x + size.x, pos.y), uv, color),
            Vertex::new(Vec2::new(pos.x, pos.y + size.y), uv, color),
            Vertex::new(pos + size, uv, color),
        ]);

        // TODO: join
        self.frame.draw_ops.push(DrawOp::DrawArrays(6));
    }

    pub fn fill_text(&mut self, text: &Text, pos: Vec2) {
        let color = self.state().fill_color;
        let mut count = 0;

        for &GlyphPos { ref glyph, pos: gpos } in text.glyphs() {
            let CachedGlyph { rect, uv } = self.glyph_cache.use_glyph(/* &text.font(),*/ glyph.clone());

            let uv = Vec2::ZERO;

            self.frame.vertices.extend_from_slice(&[
                Vertex::new(pos + gpos + rect.min, uv, color),
                Vertex::new(pos + gpos + Vec2::new(rect.min.x, rect.max.y), uv, color),
                Vertex::new(pos + gpos + Vec2::new(rect.max.x, rect.min.y), uv, color),
                Vertex::new(pos + gpos + Vec2::new(rect.min.x, rect.max.y), uv, color),
                Vertex::new(pos + gpos + Vec2::new(rect.max.x, rect.min.y), uv, color),
                Vertex::new(pos + gpos + rect.max, uv, color),
            ]);

            count += 1;
        }

        self.frame.draw_ops.push(DrawOp::DrawArrays(count * 6))
    }

    // TODO: stroke_text()
}

#[derive(Debug, Clone)]
struct State {
    fill_color: RGBA8,
    opacity: f32,
}

impl State {
    const DEFAULT: Self = Self {
        fill_color: [0, 0, 0, 255],
        opacity: 1.,
    };
}

#[derive(Debug)]
pub struct Frame {
    //pub viewport_rect: [i32; 4],
    pub vertices: Vec<Vertex>,
    pub draw_ops: Vec<DrawOp>,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            vertices: Vec::with_capacity(1024),
            draw_ops: Vec::with_capacity(32),
        }
    }
}

#[derive(Debug)]
pub enum DrawOp {
    TexData(TexData),

    // TODO: scissor_rect, texture, ?
    DrawArrays(u32),
}

#[derive(Debug, Clone)]
pub struct TexData {
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<RGBA8>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub xy: Vec2,
    pub uv: Vec2,
    pub color: RGBA8,
}

impl Vertex {
    pub const fn new(xy: Vec2, uv: Vec2, color: RGBA8) -> Self {
        Self { xy, uv, color }
    }
}
