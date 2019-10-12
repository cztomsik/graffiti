#![allow(non_snake_case)]

use crate::commons::{Pos, Bounds, SurfaceId};
use std::collections::BTreeMap;
use miniserde::{json, Deserialize, Serialize};

// TODO: currently it does more than intended so either rename it or
// split it...

/// Text layout algo
///
/// Provides glyph positions to box_layout & renderer
///
/// Text is first laid out without `max_width` wrapping
/// because in a lot of cases it will be enough. Wrapping is then
/// done (if necessary) during box_layout's measure
///
/// TODO: scaling could be done in vertex shader
/// (not sure if worth but it could save some FP which raspi is not good at)
pub struct TextLayout {
    layouts: BTreeMap<SurfaceId, TextLayoutState>,
    // TODO: more fonts, ttf
    font_glyphs: BTreeMap<char, FontGlyph>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Text {
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
        let str = include_str!("../resources/font.json");
        let font: MsdfFont = json::from_str(&str).expect("invalid font");

        let mut font_glyphs = BTreeMap::new();

        for c in font.chars {
          font_glyphs.insert(std::char::from_u32(c.id).expect("not a char"), FontGlyph {
              offset_y: c.yoffset / font.info.size,
              size: Pos::new(c.width / font.info.size, c.height / font.info.size),

              // TODO: move close to atlasing once it is ready
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
            layouts: BTreeMap::new(),
        }
    }

    pub fn set_text(&mut self, surface: SurfaceId, text: Option<Text>) {
        match text {
            None => {
                self.layouts.remove(&surface);
            }
            Some(text) => {
                let layout = self.layout_text(&text);
                self.layouts.insert(surface, layout);
            }
        }
    }

    // do initial layout, \n is respected but any other wrapping is
    // left to be done later
    //
    // not sure if it will work out but the idea is that basic
    // layout can be done even before max_width is known
    //
    // it could be noticable with more complex text shaping 
    // and maybe some of this can be done in parallel also
    //
    // when measure is called, we can compute breaks quickly and
    // return the new size so the layout can continue
    //
    // we could also start building (in other thread?)
    // the buffer with glyphs, because position & color is in uniforms
    // and can be returned
    fn layout_text(&mut self, text: &Text) -> TextLayoutState {
        let mut glyph_ids = Vec::new();
        let mut xs = Vec::new();
        let mut break_hints = Vec::new();
        let mut breaks = Vec::new();
        let mut x = 0.;

        let mut found_space = false;
        // where the current breakpoint starts
        let mut hint = None;

        // TODO: at least have a concept of shaping
        // (different number of chars vs. glyphs depending on their neighbours)

        // all glyphs are kept because of text selection
        for (i, ch) in text.text.chars().enumerate() {
            // TODO: atlasing
            let glyph_id = ch;
            let font_glyph = self.font_glyphs.entry(glyph_id).or_insert_with(|| {
              debug!("no glyph {:?}", glyph_id);
              MISSING_GLYPH
            });

            glyph_ids.push(glyph_id);
            xs.push(x);

            // TODO: FSM could be (a bit) more readable
            // but it's not that hard, it just adds hint after each space
            // ignoring any adjacent whitespace
            if ch == ' ' {
                if !found_space {
                    found_space = true;

                    if let Some(i) = hint {
                        break_hints.push((i, x));
                        hint = None;
                    }
                }
            } else if ch == '\n' {
                if let Some(i) = hint {
                    break_hints.push((i, x));
                    hint = None;
                }

                break_hints.push((i, std::f32::MAX));
                breaks.push(i);
                found_space = false;
            } else {
                if found_space {
                    hint = Some(i);
                    found_space = false;
                }
            }

            x += font_glyph.advance * text.font_size;
        }

        if let Some(i) = hint {
            break_hints.push((i, x));
        }

        TextLayoutState {
            font_size: text.font_size,
            line_height: text.line_height,

            width: x,

            glyph_ids,
            xs,
            break_hints,
            breaks,
        }
    }

    /// Wrap/reflow existing text layout to a new max_width
    /// and report a new size
    ///
    /// Expected to be called during measure.
    /// If the `Text` is changed & relayouted, wrapping is reset but
    /// the box layout should again call measure which should again
    /// call the `wrap` so it should be fine (if the wrap is necessary at all)
    pub fn wrap(&mut self, surface: SurfaceId, max_width: f32) -> (f32, f32) {
        let layout = self.layouts.get_mut(&surface).expect("not a text");

        // TODO: skip if possible (up to date)

        // TODO: stretch calls measure multiple times, which is not what we expect
        // first time it's with real value but second and third time it's unconstrained
        if max_width != std::f32::MAX {
            // TODO: other branches
            layout.width = 0.;
            layout.breaks.clear();

            let mut offset = 0.;

            // go through hints, make a break each time it overflows
            for (i, xend) in &layout.break_hints {
                if (xend - offset) >= max_width {
                    let line_width = layout.xs[*i] - offset;

                    if line_width > layout.width {
                        layout.width = line_width;
                    }

                    layout.breaks.push(*i);
                    offset = layout.xs[*i];
                }
            }

            if layout.breaks.is_empty() {
                if let Some(x) = layout.xs.last() {
                    layout.width = *x;
                }
            }
        }


        (layout.width, (layout.breaks.len() + 1) as f32 * layout.line_height)
    }

    // TODO: align center
    // (align right could be done in vertex shader)
    pub fn get_glyphs(&self, surface: SurfaceId) -> impl Iterator<Item = GlyphInstance> + '_ {
        let layout = self.layouts.get(&surface).expect("not a text");

        let mut offset = 0.;
        let mut y = -layout.line_height;
        let mut next_break = 0;
        let mut breaks = layout.breaks.iter();

        layout.xs.iter().enumerate().map(move |(i, x)| {
            if i == next_break {
                offset = *x;
                y += layout.line_height;
                next_break = *breaks.next().unwrap_or(&std::usize::MAX);
            }

            let font_glyph = self.font_glyphs.get(&layout.glyph_ids[i]).expect("glyph");

            let a = Pos::new(x - offset, y + font_glyph.offset_y * layout.font_size);

            GlyphInstance {
                bounds: Bounds { a, b: font_glyph.size.mul(layout.font_size).relative_to(a) },
                coords: font_glyph.coords
            }
        })
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
pub struct TextLayoutState {
    font_size: f32,
    line_height: f32,

    width: f32,

    // what glyphs to render
    glyph_ids: Vec<char>,

    // x of each glyph when on single line
    xs: Vec<f32>,

    // index + x of the end
    break_hints: Vec<(usize, f32)>,

    // indices
    breaks: Vec<usize>,
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