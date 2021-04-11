// text-layout
// x two-phase (shape + single-line layout when text/style changes)
// x measure(max_width)
// x for_each_glyph(rect, f)

use super::{Font, Glyph, GlyphId, ScaleFont, Vec2, AABB, SANS_SERIF_FONT};
use std::cell::{Ref, RefCell};

pub struct Text {
    text: String,
    style: TextStyle,

    single_line: RefCell<Option<SingleLine>>,
}

impl Text {
    pub fn new(text: &str, style: &TextStyle) -> Self {
        Self {
            text: text.to_owned(),
            style: style.clone(),
            single_line: Default::default(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_owned();
        self.single_line.replace(None);
    }

    pub fn style(&self) -> &TextStyle {
        &self.style()
    }

    pub fn set_style(&mut self, style: &TextStyle) {
        self.style = style.clone();
        self.single_line.replace(None);
    }

    // TODO: check/refactor (it's old code)
    // TODO: start_x for inline-layout (some el follows on the same line)
    pub fn measure(&self, max_width: f32 /* start_x */) -> (f32, f32) {
        let &SingleLine {
            width,
            ref xglyphs,
            ref break_hints,
        } = &*self.single_line();

        if width == 0. {
            return (0., 0.);
        }

        let (mut width, mut offset, mut breaks) = (0., 0., 0);

        // go through the hints, make a break each time it overflows
        if !break_hints.is_empty() {
            for (i, xend) in break_hints {
                if ((xend * self.style.font_size) - offset) > max_width {
                    let x = xglyphs[*i].0 * self.style.font_size;
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
            return (width * self.style.font_size, self.style.line_height);
        }

        (width, (breaks + 1) as f32 * self.style.line_height)
    }

    // TODO: check/refactor (it's old code)
    pub fn for_each_glyph<F: FnMut(GlyphPos)>(&self, rect: AABB /* start_x */, mut f: F) {
        let scale_font = SANS_SERIF_FONT.as_scaled(self.style.font_size);
        let layout = self.single_line();

        let (mut y, mut offset, mut hints, mut next_hint) =
            (rect.min.y, rect.min.x, layout.break_hints.iter().copied(), (0, 0.));

        for (i, &(x, glyph_id)) in layout.xglyphs.iter().enumerate() {
            let x = rect.min.x + x;

            // start of the next word/hint
            if i == next_hint.0 {
                if ((next_hint.1) - offset) > rect.max.x {
                    offset = x;
                    y += self.style.line_height;
                }

                // advance
                next_hint = hints.next().unwrap_or((std::usize::MAX, 0.));
            }

            f(GlyphPos {
                glyph: glyph_id.with_scale(scale_font.scale()),
                pos: Vec2::new(x, y),
            })
        }
    }

    fn single_line(&self) -> Ref<SingleLine> {
        if !self.single_line.borrow().is_some() {
            let scale_font = SANS_SERIF_FONT.as_scaled(self.style.font_size);

            let mut xglyphs = Vec::new();
            let mut break_hints = Vec::new();
            let mut x = 0.;
            let mut found_space = false;

            // where the current breakpoint starts
            let mut hint = None;

            // TODO: shape
            for (i, ch) in self.text.chars().enumerate() {
                let glyph_id = scale_font.glyph_id(ch);
                xglyphs.push((x, glyph_id));

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
                    found_space = false;
                } else if found_space {
                    hint = Some(i);
                    found_space = false;
                }

                x += scale_font.h_advance(glyph_id);
            }

            if let Some(i) = hint {
                break_hints.push((i, x));
            }

            self.single_line.replace(Some(SingleLine {
                width: x,
                xglyphs,
                break_hints,
            }));
        }

        Ref::map(self.single_line.borrow(), |o| o.as_ref().unwrap())
    }
}

struct SingleLine {
    width: f32,

    // (x, glyph_id) of each glyph when on single line
    xglyphs: Vec<(f32, GlyphId)>,

    // word boundaries
    // (start_index, x of the glyph after)
    break_hints: Vec<(usize, f32)>,
}

#[derive(Debug)]
pub struct GlyphPos {
    pub glyph: Glyph,
    pub pos: Vec2,
}

#[derive(Debug, Clone)]
pub struct TextStyle {
    //pub font_query: FontQuery,
    pub font_size: f32,
    pub line_height: f32,
    //pub letter_spacing: f32,
    //pub word_spacing: f32,
    // TODO: white-space
    pub align: TextAlign,
}

impl TextStyle {
    pub const DEFAULT: Self = Self {
        font_size: 16.,
        line_height: 20.,
        align: TextAlign::Left,
    };
}

#[derive(Debug, Clone)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measure() {
        let text = Text::new("X XX XXX XXXX", &TextStyle::DEFAULT);

        assert_eq!(text.measure(30.).1, 90.);
        assert_eq!(text.measure(60.).1, 60.);
        assert_eq!(text.measure(100.).1, 30.);
    }

    #[test]
    fn glyphs() {
        let text = Text::new("Hello", &TextStyle::DEFAULT);
        text.for_each_glyph(AABB::ZERO, |g| println!("{:?}", g));
    }
}
