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
        &self.style
    }

    pub fn set_style(&mut self, style: &TextStyle) {
        self.style = style.clone();
        self.single_line.replace(None);
    }

    // TODO: check/refactor (it's old code)
    // TODO: start_x for inline-layout (some el follows on the same line)
    pub fn measure(&self, max_width: f32 /* start_x */) -> (f32, f32) {
        let &SingleLine {
            ref xglyphs,
            ref break_hints,
        } = &*self.single_line();

        // empty or white-space
        if xglyphs.is_empty() {
            return (0., 0.);
        }

        let (mut width, mut offset, mut breaks) = (0., 0., 0);

        // go through the hints, make a break each time it overflows
        if !break_hints.is_empty() {
            for (i, xend) in break_hints {
                if (xend - offset) > max_width {
                    let x = xglyphs[*i].0;
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
            return (xglyphs.last().unwrap().0 * self.style.font_size, self.style.line_height);
        }

        (width, (breaks + 1) as f32 * self.style.line_height)
    }

    // TODO: check/refactor (it's old code)
    pub fn for_each_glyph<F: FnMut(GlyphPos)>(&self, rect: AABB /* start_x */, mut f: F) {
        let scale_font = SANS_SERIF_FONT.as_scaled(self.style.font_size);
        let single_line = self.single_line();
        let baseline = scale_font.height() + scale_font.descent();

        let mut y = rect.min.y + self.style.line_height - (self.style.line_height - baseline) / 2.;
        let mut offset = 0.;
        let mut hints = single_line.break_hints.iter().copied();
        let mut next_hint = (0, 0.);

        for &(x, glyph_id) in single_line.xglyphs.iter() {
            // next word/hint
            if x > next_hint.1 {
                if (rect.min.x + next_hint.1 - offset) > rect.max.x {
                    offset = x;
                    y += self.style.line_height;
                }

                // advance
                next_hint = hints.next().unwrap_or((std::usize::MAX, 0.));
            }

            f(GlyphPos {
                glyph: glyph_id.with_scale(scale_font.scale()),
                pos: Vec2::new(rect.min.x + x - offset, y),
            })
        }
    }

    fn single_line(&self) -> Ref<SingleLine> {
        if !self.single_line.borrow().is_some() {
            let scale_font = SANS_SERIF_FONT.as_scaled(self.style.font_size);

            let mut xglyphs = Vec::new();
            let mut break_hints = Vec::new();
            let mut x = 0.;
            let mut in_space = false;

            // where the current breakpoint starts
            let mut hint = None;

            // TODO: shape
            for ch in self.text.chars() {
                // TODO: FSM could be (a bit) more readable
                // but it's not that bad, it just adds hint after each space
                // ignoring any adjacent whitespace
                if is_space(ch, self.style.pre) {
                    if !in_space {
                        in_space = true;
                        x += scale_font.h_advance(scale_font.glyph_id(' '));

                        if let Some(i) = hint {
                            break_hints.push((i, x));
                            hint = None;
                        }
                    }
                // only if pre == false
                } else if ch == '\n' {
                    break_hints.push((xglyphs.len(), std::f32::MAX));
                    xglyphs.push((x, scale_font.glyph_id(' ')));
                    in_space = false;
                } else {
                    if in_space {
                        hint = Some(xglyphs.len());
                        in_space = false;
                    }

                    let glyph_id = scale_font.glyph_id(ch);
                    xglyphs.push((x, glyph_id));
                    x += scale_font.h_advance(glyph_id);
                }
            }

            if let Some(i) = hint {
                break_hints.push((i, x));
            }

            self.single_line.replace(Some(SingleLine { xglyphs, break_hints }));
        }

        Ref::map(self.single_line.borrow(), |o| o.as_ref().unwrap())
    }
}

fn is_space(ch: char, preserve: bool) -> bool {
    ch == ' ' || !preserve && (ch == '\t' || ch == '\n' || ch == '\r')
}

#[derive(Debug)]
struct SingleLine {
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
    pub align: TextAlign,
    pub pre: bool,
}

impl TextStyle {
    pub const DEFAULT: Self = Self {
        font_size: 16.,
        line_height: 20.,
        align: TextAlign::Left,
        pre: false,
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
