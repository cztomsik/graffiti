use super::ImageId;
use crate::commons::Pos;
use std::fmt::{self, Debug, Formatter};

// value types
// part of the public interface but not necessarily how it's stored internally
// often similar to respective CSS properties but not the same
// (dimensions are absolute, granularity is different, etc.)

// TODO: should be (f32, f32)
#[derive(Debug, Clone, Copy)]
pub struct BorderRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
}

#[derive(Debug, Clone, Copy)]
pub struct OutlineShadow {
    pub offset: Pos,
    pub blur: f32,
    pub spread: f32,
    pub color: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct Outline {
    pub width: f32,
    pub style: OutlineStyle,
    pub color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutlineStyle {
    Solid,
}

/// Packed color
/// note that u32 could improve interop or CPU but GPU is float-only
/// and bitwise ops are slow so it still needs to be unpacked during
/// `VertexAttribPointer()` as it is done now
#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const TRANSPARENT: Color = Self { r: 0, g: 0, b: 0, a: 0 };
    pub const BLACK: Color = Self { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Self { r: 255, g: 255, b: 255, a: 255 };

    // just to make testing & prototyping a bit easier
    pub const RED: Color = Self { r: 255, g: 0, b: 0, a: 255 };
    pub const GREEN: Color = Self { r: 0, g: 255, b: 0, a: 255 };
    pub const BLUE: Color = Self { r: 0, g: 0, b: 255, a: 255 };
    pub const YELLOW: Color = Self { r: 255, g: 255, b: 0, a: 255 };
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.a == 255 {
            write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            write!(f, "#{:02x}{:02x}{:02x}#{:02x}", self.r, self.g, self.b, self.a)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BackgroundImage {
    Image { image: ImageId },
    LinearGradient {},
    RadialGradient {},
}

#[derive(Debug, Clone, Copy)]
pub struct Border {
    pub top: Option<BorderSide>,
    pub right: Option<BorderSide>,
    pub bottom: Option<BorderSide>,
    pub left: Option<BorderSide>,
}

#[derive(Debug, Clone, Copy)]
pub struct BorderSide {
    pub width: f32,
    pub style: BorderStyle,
    pub color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BorderStyle {
    None,
    Solid,
}

#[derive(Debug, Clone, Copy)]
pub struct InsetShadow {
    pub offset: Pos,
    pub blur: f32,
    pub spread: f32,
    pub color: Color,
}
