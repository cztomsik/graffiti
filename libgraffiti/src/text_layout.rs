#![allow(non_snake_case)]

use crate::commons::{Pos, Bounds, SurfaceId, Color};
use std::collections::BTreeMap;
use miniserde::{json, Deserialize, Serialize};

// TODO: currently it does more than intended so either rename it or
// split it...

/// Text layout algo
///
/// Should lay glyphs on each `Text` change without any wrapping
/// because in a lot of cases it will be enough
///
/// The box layout should call `measure_text` during its `calculate`
/// which in turn should call `wrap` if it`s needed.
pub struct TextLayout {
    metas: BTreeMap<SurfaceId, Meta>,
    // TODO: more fonts, ttf
    font_glyphs: BTreeMap<u32, FontGlyph>,
    glyphs: BTreeMap<SurfaceId, Vec<GlyphInstance>>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Text {
    pub color: Color,
    pub font_size: f32,

    pub line_height: f32,

    pub align: TextAlign,
    pub text: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

impl TextLayout {
    pub fn new() -> Self {
        let str = std::fs::read_to_string("../font.json").expect("font file");
        let font: MsdfFont = json::from_str(&str).expect("invalid font");

        let mut font_glyphs = BTreeMap::new();

        for c in font.chars {
          font_glyphs.insert(c.id, FontGlyph {
              offset_y: c.yoffset,
              size: Pos::new(c.width / font.info.size, c.height / font.info.size),
              coords: Bounds {
                a: Pos::new(c.x / font.common.scaleW, c.y / font.common.scaleH),
                b: Pos::new((c.x + c.width) / font.common.scaleW, (c.y + c.height) / font.common.scaleH),
              },
              advance: c.xadvance / font.info.size,
          });
        }

        silly!("glyphs {:#?}", &font_glyphs);

        TextLayout {
            font_glyphs,
            metas: BTreeMap::new(),
            glyphs: BTreeMap::new()
        }
    }

    pub fn set_text(&mut self, surface: SurfaceId, text: Option<Text>) {
        match text {
            None => {
                self.metas.remove(&surface);
                self.glyphs.remove(&surface);
            }
            Some(text) => {
                let (meta, glyphs) = self.layout_text(&text);

                self.metas.insert(surface, meta);
                self.glyphs.insert(surface, glyphs);
            }
        }
    }

    fn layout_text(&self, text: &Text) -> (Meta, Vec<GlyphInstance>) {
        let mut size = (0., text.line_height);
        let mut pos = Pos::zero();

        let glyphs = text.text.chars().into_iter().map(|c| {
            if c == '\n' {
                pos.x = 0.;
                pos.y += text.line_height;
            }

            // TODO
            let glyph_id = c as u32;
            let font_glyph = self.font_glyphs.get(&glyph_id).unwrap_or_else(|| {
              debug!("no glyph {}", glyph_id);

              &MISSING_GLYPH
            });

            // TODO: read from font
            let base = 38.964 / 42.;
            let a = Pos::new(pos.x, pos.y + (font_glyph.offset_y / 42. * text.font_size) + base * text.font_size - font_glyph.size.y * text.font_size);

            let glyph = GlyphInstance {
                bounds: Bounds { a, b: font_glyph.size.mul(text.font_size).relative_to(a) },
                coords: font_glyph.coords,
            };

            pos.x += font_glyph.advance * text.font_size;

            glyph
        }).collect();

        // TODO: wrap
        // TODO: good for now but we should use glyph width for the last char on each line
        size.0 = pos.x;
        size.1 = pos.y + text.line_height;

        let meta = Meta {
            size,
            initial_width: size.0
        };

        silly!("{:#?} {:#?}", &meta, &glyphs);

        (meta, glyphs)
    }

    /// Wrap/reflow existing text layout to a new max_width
    /// should skip if the `max_width` is `None` or bigger than current width
    ///
    /// Expected to be called during measure.
    /// If the `Text` is changed wrapping is reset but
    /// the box layout should again call measure which should again
    /// call the `wrap` so it should be fine (if the wrap is necessary at all)
    pub fn wrap(&mut self, _surface: SurfaceId, _max_width: Option<f32>) {
        // TODO
    }

    pub fn get_size(&self, surface: SurfaceId) -> (f32, f32) {
        self.metas.get(&surface).expect("not a text").size
    }

    pub fn get_glyphs(&self, surface: SurfaceId) -> &[GlyphInstance] {
        self.glyphs.get(&surface).expect("not a text")
    }

    // other expected use-cases (not necessarily the sole responsibility of this but related)
    // - get word boundaries at (x, y) to select word
    // - get selection boundaries from (x, y) to (x, y) during selection
    // - set cursor closest to (x, y)
    // - move cursor with keyboard arrows, respecting wrapping
    // - select next word

}

#[derive(Debug)]
pub struct FontGlyph {
  pub size: Pos,
  pub coords: Bounds,
  pub advance: f32,
  pub offset_y: f32,
}

#[derive(Debug)]
pub struct GlyphInstance {
    pub bounds: Bounds,
    pub coords: Bounds,
}

#[derive(Debug)]
pub struct Meta {
    size: (f32, f32),
    initial_width: f32,
}

// msdf parsing

#[derive(Deserialize, Serialize, Debug)]
pub struct MsdfFont {
    info: MsdfInfo,
    common: MsdfCommonInfo,
    chars: Vec<MsdfChar>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MsdfInfo {
    size: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MsdfCommonInfo {
    scaleW: f32,
    scaleH: f32,
    base: f32,
    lineHeight: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MsdfChar {
    id: u32,
    x: f32,
    y: f32,
    xoffset: f32,
    yoffset: f32,
    width: f32,
    height: f32,
    xadvance: f32,
}

// ::default() cannot be used to initialize static :-/
const MISSING_GLYPH: FontGlyph = FontGlyph { offset_y: 0., size: Pos { x: 0., y: 0. }, coords: Bounds { a: Pos { x: 0., y: 0. }, b: Pos { x: 0., y: 0. } }, advance: 0. };