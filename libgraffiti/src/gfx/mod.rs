mod atlas;
mod backend;
mod canvas;
mod font;
mod glyph_cache;
mod text;

pub use ochre::{PathCmd, Transform, Vec2};

pub use {atlas::*, backend::*, canvas::*, font::*, glyph_cache::*, text::*};

use std::ops::Add;

#[allow(clippy::upper_case_acronyms)]
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
