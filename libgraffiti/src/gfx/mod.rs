mod atlas;
mod backend;
mod canvas;
mod font;
mod glyph_cache;
mod text;

pub use ochre::{PathCmd, Transform, Vec2};

pub use {
  backend::{GlBackend, RenderBackend},
  canvas::{Canvas, Frame, TexData, UvRect, DrawOp, Vertex, RGBA8},
  font::{Font, Glyph, GlyphId, ScaleFont},
  text::{TextStyle, Text, GlyphPos, Paragraph, ParagraphBuilder}
};

pub(crate) use font::SANS_SERIF_FONT;

pub trait Drawable {
  fn draw(&self, canvas: &mut Canvas, origin: Vec2);
}

use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AABB {
  pub min: Vec2,
  pub max: Vec2,
}

impl AABB {
  pub const ZERO: Self = Self {
    min: Vec2 { x: 0., y: 0. },
    max: Vec2 { x: 0., y: 0. },
  };

  pub const fn new(min: Vec2, max: Vec2) -> Self {
    Self { min, max }
  }
}

impl Add<Vec2> for AABB {
  type Output = Self;

  fn add(self, pos: Vec2) -> Self {
    Self {
      min: self.min + pos,
      max: self.max + pos,
    }
  }
}
