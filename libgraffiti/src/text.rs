// x load face from ttf (and make it available under given name)
// x resolve font face (with fallback to default font)
// x no system fonts
// x no unloading
// x no font-family stacks
// x have concept of shaping
// x two-phase layout
//   x prepare (shape + single-line layout) when text/font changes
//   x measure/get_glyphs using "text style" with size, align, spacing, ... which can change between calls
// x texts are owned resources
//   - possible to find out what glyphs are used
//   - rasterize differently depending on the font size
//     (msdf for bigger sizes, downscaled alpha otherwise)

#![allow(non_snake_case)]

use crate::commons::{Bounds, Id, Pos};
use graffiti_stb::{stbtt_FindGlyphIndex, stbtt_GetGlyphBox, stbtt_GetGlyphHMetrics, stbtt_InitFont, stbtt_ScaleForPixelHeight, stbtt_fontinfo};
use std::os::raw::c_int;

#[derive(Debug, Clone, Copy)]
pub struct TextStyle {
    pub font: FontId,
    pub font_size: f32,
    pub line_height: f32,
    pub align: TextAlign,
    // TODO: spacing, direction, lang, ?
}

#[derive(Debug, Clone, Copy)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

pub struct TextEngine {
    fonts: Vec<Font>,
    texts: Vec<Text>,
}

impl TextEngine {
    pub const DEFAULT_FONT: FontId = FontId::new(0);

    pub fn new() -> Self {
        let mut engine = Self { fonts: Vec::new(), texts: Vec::new() };

        engine.load_font("sans-serif", Box::new(*ROBOTO_TTF));

        engine
    }

    // no unloading for now
    pub fn load_font(&mut self, name: &str, ttf_data: Box<[u8]>) -> FontId {
        let mut info;

        unsafe {
            info = std::mem::MaybeUninit::<stbtt_fontinfo>::uninit().assume_init();
            stbtt_InitFont(&mut info, ttf_data.as_ptr(), 0);
        }

        self.fonts.push(Font {
            name: name.to_string(),

            space_glyph: unsafe { stbtt_FindGlyphIndex(&info, ' ' as c_int) },
            newline_glyph: unsafe { stbtt_FindGlyphIndex(&info, '\n' as c_int) },

            info,
            // according to stb docs, the buffer has to be kept in memory
            _ttf_data: ttf_data,
        });

        FontId::new(self.fonts.len() - 1)
    }

    // TODO: consider some FontQuery { weight, style, variant } but for now
    // we can just use face name with some suffix (computed elsewhere)
    pub fn resolve_font(&self, name: &str) -> FontId {
        if let Some(index) = self.fonts.iter().position(|f| f.name == name) {
            return FontId::new(index);
        }

        Self::DEFAULT_FONT
    }

    pub fn create_text(&mut self) -> TextId {
        self.texts.push(Text::EMPTY);

        TextId::new(self.texts.len() - 1)
    }

    pub fn set_text_style(&mut self, text: TextId, style: &TextStyle) {
        let prev = std::mem::replace(&mut self.texts[text].style, *style);

        if prev.font != style.font {
            self.rebuild_text(text);
        }
    }

    pub fn set_text_chars(&mut self, text: TextId, chars: String) {
        self.texts[text].chars = chars;

        self.rebuild_text(text);
    }

    pub fn measure_text(&self, text: TextId, max_width: f32) -> (f32, f32) {
        self.texts[text].measure(max_width)
    }

    // TODO: https://iamvdo.me/en/blog/css-font-metrics-line-height-and-vertical-align
    // TODO: align center/right
    // TODO: we don't need lines/runs for now
    //       (and maybe it's enough to have .font in PosGlyph)
    pub fn get_text_glyphs(&self, text: TextId, width: f32) -> impl Iterator<Item = PosGlyph> + '_ {
        let Text {
            style, glyph_ids, xs, break_hints, ..
        } = &self.texts[text];
        let TextStyle { font, font_size, line_height, .. } = style;

        // TODO: find out from which offset_y is computed (x_height/cap_height/ascender/?)
        // and remove this magic factor to make it font independent
        //let mut y = -style.line_height + ((style.line_height - (1.25 * style.font_size)) / 2.);
        let mut y = 0.;
        let mut offset = 0.;
        let mut hints = break_hints.iter();
        let mut next_hint = &(0, 0.);

        let scale = unsafe { stbtt_ScaleForPixelHeight(&self.fonts[*font].info, style.font_size) };

        // TODO: store it in runs and return correct font for each glyph
        xs.iter().enumerate().map(move |(i, x)| {
            let x = *x * font_size;

            // start of the next word/hint
            if i == next_hint.0 {
                if ((next_hint.1 * font_size) - offset) > width {
                    offset = x;
                    y += line_height;
                }

                // advance
                next_hint = hints.next().unwrap_or(&(std::usize::MAX, 0.));
            }

            let mut bounds = (0, 0, 0, 0);

            unsafe { stbtt_GetGlyphBox(&self.fonts[*font].info, glyph_ids[i], &mut bounds.0, &mut bounds.1, &mut bounds.2, &mut bounds.3) };

            PosGlyph {
                bounds: Bounds {
                    a: Pos::new(bounds.0 as f32 * scale, bounds.1 as f32 * scale),
                    b: Pos::new(bounds.2 as f32 * scale, bounds.3 as f32 * scale),
                }
                .translate(Pos::new(x - offset, y)),
            }
        })
    }

    // do shaping & initial single-line layout
    // even before max_width is known
    //
    // when measure is called, we can compute breaks quickly and
    // return the new size so the layout can continue
    //
    // TODO: direction, lang
    // TODO: we could do kerning too
    fn rebuild_text(&mut self, text: TextId) {
        let Text {
            style,
            chars,
            glyph_ids,
            xs,
            break_hints,
            single_line_width,
        } = &mut self.texts[text];
        let Font {
            ref info, space_glyph, newline_glyph, ..
        } = self.fonts[style.font];

        let scale = unsafe { stbtt_ScaleForPixelHeight(info, 1.) };
        let glyphs = Self::shape_text(&info, chars.chars());

        // clear
        *glyph_ids = Vec::new();
        *xs = Vec::new();
        *break_hints = Vec::new();

        // acc state
        let mut x = 0.;
        let mut found_space = false;
        // where the current breakpoint starts
        let mut hint = None;

        // TODO: offset
        for (i, (glyph_id, _offset, advance)) in glyphs.enumerate() {
            silly!("glyph #{} {}", glyph_id, advance.x);
            glyph_ids.push(glyph_id);
            xs.push(x);

            // FSM could be (a bit) more readable
            // but it's not that bad, it just adds hint after each space
            // ignoring any adjacent whitespace
            if glyph_id == space_glyph {
                if !found_space {
                    found_space = true;

                    if let Some(i) = hint {
                        break_hints.push((i, x));
                        hint = None;
                    }
                }
            } else if glyph_id == newline_glyph {
                if let Some(i) = hint {
                    break_hints.push((i, x));
                    hint = None;
                }

                break_hints.push((i, std::f32::MAX));
                found_space = false;
            } else if found_space {
                hint = Some(i);
                found_space = false;
            }

            x += advance.x * scale;
        }

        *single_line_width = x;

        if let Some(i) = hint {
            break_hints.push((i, x));
        }
    }

    // different number of glyphs/offsets/advances depending on their neighbours
    fn shape_text<'a>(font: &'a stbtt_fontinfo, chars: impl Iterator<Item = char> + 'a) -> impl Iterator<Item = (GlyphId, Pos, Pos)> + 'a {
        // TODO: real shaping (harfbuzz or allsorts)
        chars.map(move |ch| unsafe {
            let glyph_id = stbtt_FindGlyphIndex(font, ch as c_int);
            let mut bounds = (0, 0, 0, 0);
            let mut advance = (0, 0);

            stbtt_GetGlyphBox(font, glyph_id, &mut bounds.0, &mut bounds.1, &mut bounds.2, &mut bounds.3);
            stbtt_GetGlyphHMetrics(font, glyph_id, &mut advance.0, &mut 0);

            (
                // TODO: real shaping can produce glyphs from different fonts (so it should be part of the result)
                // font,
                glyph_id,
                // offset
                Pos::new(bounds.0 as f32, bounds.1 as f32),
                // total_horiz_advance (size + advance)
                Pos::new((advance.0) as f32, advance.1 as f32),
            )
        })
    }
}

pub type FontId = Id<Font>;

pub type TextId = Id<Text>;

#[derive(Debug, Clone, Copy)]
pub struct PosGlyph {
    // font: FontId,
    //pub glyph: GlyphId,
    //pub pos: Pos,
    pub bounds: Bounds,
}

// private from here
// (pubs because of Id<T>)

pub struct Font {
    name: String,
    info: stbtt_fontinfo,
    _ttf_data: Box<[u8]>,
    space_glyph: GlyphId,
    newline_glyph: GlyphId,
}

pub struct Text {
    style: TextStyle,
    chars: String,

    // what glyphs to render
    // TODO: should be Vec of runs (font + ids + xs)
    glyph_ids: Vec<GlyphId>,

    // x of each glyph when on single line
    xs: Vec<f32>,

    // word boundaries
    // (start_index, x of the glyph after)
    break_hints: Vec<(usize, f32)>,
    single_line_width: f32,
}

impl Text {
    // useful until there's known font/text
    const EMPTY: Text = Text {
        style: TextStyle {
            font: TextEngine::DEFAULT_FONT,
            font_size: 16.,
            line_height: 30.,
            align: TextAlign::Left,
        },
        chars: String::new(),
        glyph_ids: Vec::new(),
        xs: Vec::new(),
        break_hints: Vec::new(),
        single_line_width: 0.,
    };

    // TODO: *-spacing, align
    pub fn measure(&self, max_width: f32) -> (f32, f32) {
        if self.single_line_width == 0. {
            return (0., 0.);
        }

        let TextStyle { font_size, line_height, .. } = self.style;

        let mut width = 0.;
        let mut offset = 0.;
        let mut breaks = 0;

        // go through the hints, make a break each time it overflows
        if !self.break_hints.is_empty() {
            for (i, xend) in &self.break_hints {
                if ((xend * font_size) - offset) > max_width {
                    let x = self.xs[*i] * font_size;
                    let line_width = x - offset;

                    if line_width > width {
                        width = line_width;
                    }

                    breaks += 1;
                    offset = x;
                }
            }
        }

        if breaks == 0 {
            return (self.single_line_width * font_size, line_height);
        }

        (width, (breaks + 1) as f32 * line_height)
    }

    // TODO: other expected use-cases (not necessarily the sole responsibility of this but related)
    // - get word boundaries at (x, y) to select word
    // - get selection boundaries from (x, y) to (x, y) during selection
    // - set cursor closest to (x, y)
    // - move cursor with keyboard arrows, respecting wrapping
    // - select next word
}

type GlyphId = std::os::raw::c_int;

pub const ROBOTO_TTF: &[u8; 171272] = include_bytes!("../resources/Roboto/Roboto-Regular.ttf");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        TextEngine::new();
    }

    #[test]
    fn load_resolve_font() {
        let mut te = TextEngine::new();
        let f = te.load_font("test", Box::new(*ROBOTO_TTF));

        assert_eq!(te.resolve_font("test"), f);
        assert_ne!(te.resolve_font("foo"), f);
    }

    #[test]
    fn empty_text() {
        TextEngine::new().create_text();
    }

    #[test]
    fn hello_text() {
        let mut te = TextEngine::new();
        let t = te.create_text();

        te.set_text_chars(t, "Hello".to_string());
    }

    #[test]
    fn measure() {
        let mut te = TextEngine::new();
        let t = te.create_text();

        te.set_text_chars(t, "X XX XXX XXXX".to_string());

        assert_eq!(te.measure_text(t, 30.).1, 90.);
        assert_eq!(te.measure_text(t, 60.).1, 60.);
        assert_eq!(te.measure_text(t, 100.).1, 30.);
    }

    #[test]
    fn get_glyphs() {
        let mut te = TextEngine::new();
        let t = te.create_text();

        te.set_text_chars(t, "Hello".to_string());

        //let glyphs: Vec<PosGlyph> = t.get_pos_glyphs(&ts, 100.).collect();
        //println!("{:#?}", glyphs);

        assert!(false)
    }
}
