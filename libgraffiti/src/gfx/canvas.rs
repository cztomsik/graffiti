// super-simple GPU-first 2D graphics
// x outputs (textured) vertices + draw "ops"
// x easy to integrate (and to compose)

use super::{CachedGlyph, GlyphCache, GlyphPos, PathCmd, Text, Transform, Vec2, AABB};
use ochre::{Rasterizer, TileBuilder, TILE_SIZE};
use std::ops::{Index, IndexMut};

#[allow(clippy::upper_case_acronyms)]
pub type RGBA8 = [u8; 4];

pub struct Canvas {
    frame: Frame,
    glyph_cache: GlyphCache,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            frame: Frame::new(),
            glyph_cache: GlyphCache::new(),
        }
    }

    pub fn fill_rect(&mut self, rect: AABB, color: RGBA8) {
        self.push_quad(rect, AABB::ZERO, color);
    }

    pub fn fill_path(&mut self, path: &[PathCmd], color: RGBA8) {
        println!("TODO: fill_path color");

        let mut rasterizer = Rasterizer::new();
        rasterizer.fill(path, Transform::id());
        rasterizer.finish(self);
    }

    pub fn fill_text(&mut self, text: &Text, text_rect: AABB, color: RGBA8) {
        text.for_each_glyph(text_rect, |GlyphPos { glyph, pos }| {
            let &CachedGlyph { rect, uv } = self.glyph_cache.use_glyph(/* &text.font(),*/ glyph);

            self.push_quad(rect + pos, uv, color);
        });
    }

    // TODO: stroke_text()

    fn push_quad(&mut self, rect: AABB, uv: AABB, color: RGBA8) {
        self.frame.vertices.extend_from_slice(&[
            Vertex::new(rect.min, uv.min, color),
            Vertex::new(Vec2::new(rect.min.x, rect.max.y), Vec2::new(uv.min.x, uv.max.y), color),
            Vertex::new(Vec2::new(rect.max.x, rect.min.y), Vec2::new(uv.max.x, uv.min.y), color),
            Vertex::new(Vec2::new(rect.min.x, rect.max.y), Vec2::new(uv.min.x, uv.max.y), color),
            Vertex::new(Vec2::new(rect.max.x, rect.min.y), Vec2::new(uv.max.x, uv.min.y), color),
            Vertex::new(rect.max, uv.max, color),
        ]);

        // join
        if let Some(DrawOp::DrawArrays(num)) = self.frame.draw_ops.last_mut() {
            *num += 6;
        } else {
            self.frame.draw_ops.push(DrawOp::DrawArrays(6));
        }
    }

    pub fn flush(&mut self) -> Frame {
        println!("{:?}", self.frame);

        self.frame
            .draw_ops
            .insert(0, DrawOp::TexData(self.glyph_cache.tex_data().clone()));

        std::mem::replace(&mut self.frame, Frame::new())
    }
}

impl TileBuilder for Canvas {
    fn tile(&mut self, x: i16, y: i16, data: [u8; TILE_SIZE * TILE_SIZE]) {
        let uv = self
            .glyph_cache
            .atlas
            .push(TILE_SIZE as _, TILE_SIZE as _, |tex: &mut TexData, x, y| {
                for row in 0..TILE_SIZE {
                    for col in 0..TILE_SIZE {
                        let alpha = data[row * TILE_SIZE + col];
                        tex[(x + col, y + row)] = [alpha, alpha, alpha, alpha];
                    }
                }
            })
            .unwrap();

        let min = Vec2::new(x as _, y as _);
        self.push_quad(
            AABB::new(min, min + Vec2::new(TILE_SIZE as _, TILE_SIZE as _)),
            uv,
            [255, 0, 0, 200],
        );
    }

    fn span(&mut self, x: i16, y: i16, width: u16) {
        let min = Vec2::new(x as _, y as _);
        self.fill_rect(
            AABB::new(min, min + Vec2::new(width as _, TILE_SIZE as _)),
            [255, 0, 0, 200],
        );
    }
}

#[derive(Debug)]
pub struct Frame {
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
