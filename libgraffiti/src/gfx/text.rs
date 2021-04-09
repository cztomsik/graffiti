use super::{Font, FontArc, Glyph, ScaleFont, Vec2, SANS_SERIF_FONT};

pub struct Text {
    //style: TextStyle? Atom?
    //font: Font
    //lines: Vec<Line>
    glyphs: Vec<GlyphPos>,
    size: (f32, f32)
}

impl Text {
    pub fn new(text: &str) -> Self {
        let style = &TextStyle::DEFAULT;
        let scale_font = SANS_SERIF_FONT.as_scaled(style.font_size);

        let mut x = 0.;
        let y = style.line_height;

        let mut glyphs = Vec::new();

        for c in text.chars() {
            let glyph = scale_font.scaled_glyph(c);
            let rect = scale_font.glyph_bounds(&glyph);
            let advance = scale_font.h_advance(glyph.id);

            glyphs.push(GlyphPos {
                glyph,
                pos: Vec2::new(x + rect.min.x, y),
            });

            x += advance;
        }

        Self { glyphs, size: (x, y) }
    }

    pub fn style(&self) -> &TextStyle {
        &TextStyle::DEFAULT
    }

    /*
    pub fn font(&self) -> &FontArc {
        &SANS_SERIF_FONT
    }
    */

    pub fn size(&self) -> (f32, f32) {
        self.size
    }

    // TODO: lines/runs + Iterator
    pub fn glyphs(&self) -> &[GlyphPos] {
        &self.glyphs
    }
}

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
        font_size: 32.,
        line_height: 40.,
        align: TextAlign::Left,
    };
}

#[derive(Debug, Clone)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

/*
pub struct TextLayoutKey {
    text: Atom<String>,
    style: Atom<TextStyle>,
    max_width: f32
}
*/
