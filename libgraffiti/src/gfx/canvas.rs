// super-simple GPU-first 2D graphics
// x outputs (textured) vertices + draw "ops"
// x easy to integrate (and to compose)

use super::{CachedGlyph, GlyphCache, GlyphPos, Text, Vec2, AABB};
use std::ops::{Index, IndexMut};

#[allow(clippy::upper_case_acronyms)]
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

    pub fn fill_rect(&mut self, rect: AABB) {
        let AABB { min, max } = rect;
        let color = self.state().fill_color;
        let uv = Vec2::ZERO;

        self.frame.vertices.extend_from_slice(&[
            Vertex::new(min, uv, color),
            Vertex::new(Vec2::new(max.x, min.y), uv, color),
            Vertex::new(Vec2::new(min.x, max.y), uv, color),
            Vertex::new(Vec2::new(max.x, min.y), uv, color),
            Vertex::new(Vec2::new(min.x, max.y), uv, color),
            Vertex::new(max, uv, color),
        ]);

        // TODO: join
        self.frame.draw_ops.push(DrawOp::DrawArrays(6));
    }

    pub fn fill_text(&mut self, text: &Text, rect: AABB) {
        let color = self.state().fill_color;
        let mut count = 0;

        text.for_each_glyph(rect, |GlyphPos { glyph, pos }| {
            let CachedGlyph { rect, uv } = self.glyph_cache.use_glyph(/* &text.font(),*/ glyph);

            self.frame.vertices.extend_from_slice(&[
                Vertex::new(pos + rect.min, uv.min, color),
                Vertex::new(
                    pos + Vec2::new(rect.min.x, rect.max.y),
                    Vec2::new(uv.min.x, uv.max.y),
                    color,
                ),
                Vertex::new(
                    pos + Vec2::new(rect.max.x, rect.min.y),
                    Vec2::new(uv.max.x, uv.min.y),
                    color,
                ),
                Vertex::new(
                    pos + Vec2::new(rect.min.x, rect.max.y),
                    Vec2::new(uv.min.x, uv.max.y),
                    color,
                ),
                Vertex::new(
                    pos + Vec2::new(rect.max.x, rect.min.y),
                    Vec2::new(uv.max.x, uv.min.y),
                    color,
                ),
                Vertex::new(pos + rect.max, uv.max, color),
            ]);

            count += 1;
        });

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

impl Index<(usize, usize)> for TexData {
    type Output = RGBA8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.pixels[y * self.width as usize + x]
    }
}

impl IndexMut<(usize, usize)> for TexData {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.pixels[y * self.width as usize + x]
    }
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
