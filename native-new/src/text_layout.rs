use std::collections::BTreeMap;
use crate::generated::{UpdateSceneMsg, StyleProp, SurfaceId, Text};

/// Text layout algo
///
/// Should lay glyphs on each `Text` change without any wrapping
/// because in a lot of cases it will be enough
///
/// The box layout should call `measure_text` during its `calculate`
/// which in turn should call `wrap` if it`s needed.
pub struct TextLayout {
    font: font_kit::font::Font,
    metas: BTreeMap<SurfaceId, Meta>,
    glyphs: BTreeMap<SurfaceId, Vec<GlyphInstance>>
}

impl TextLayout {
    pub fn new() -> Self {
        let font = SystemSource::new()
            .select_by_postscript_name("ArialMT")
            .unwrap()
            .load()
            .unwrap();

        TextLayout {
            font,
            metas: BTreeMap::new(),
            glyphs: BTreeMap::new()
        }
    }

    fn update_scene(&mut self, msgs: &[UpdateSceneMsg]) {
        for m in msgs {
            match m {
                UpdateSceneMsg::SetStyleProp { surface, prop: StyleProp::Text(t) } => {
                    match t {
                        None => {
                            self.metas.remove(surface);
                            self.glyphs.remove(surface);
                        }
                        Some(text) => {
                            let (meta, glyphs) = self.layout_text(text);

                            self.metas.insert(*surface, meta);
                            self.glyphs.insert(*surface, glyphs);
                        }
                    }
                },
                _ => {}
            }
        }
    }

    fn layout_text(&self, text: &Text) -> (Meta, Vec<Glyph>) {
        let mut size = (0., text.line_height);
        let mut x = 0.;
        let mut y = 0.;

        // TODO: find our how costy it is
        let scale = text.font_size / (self.font.metrics().units_per_em as f32);

        let glyphs = text.text.chars().into_iter().map(|c| {
            if c == '\n' {
                x = 0.;
                y += text.line_height;
            }

            let glyph_id = self.font.glyph_for_char(c).unwrap_or(0);
            let advance = match self.font.advance(glyph_id) {
                Ok(v) => (v.x * scale, v.y * scale),
                Err(_e) => (0., 0.)
            };

            let glyph = GlyphInstance {
                glyph_id,
                x,
                y,
            };

            x += advance.0;
            y += advance.1;

            glyph
        }).collect();

        // TODO: wrap
        // TODO: good for now but we should use glyph width for the last char on each line
        size.0 = x;
        size.1 = y + text.line_height;

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
    fn wrap(&mut self, surface: SurfaceId, max_width: Option<f32>) {
        // TODO
    }

    fn get_size(&self, surface: SurfaceId) -> (f32, f32) {
        self.metas.get(&surface).expect("not a text").size
    }

    fn get_glyphs(&self, surface: SurfaceId) -> &[Glyph] {
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
pub struct GlyphInstance {
    pub glyph_id: u32,
    pub x: f32,
    pub y: f32,
}

use font_kit::source::SystemSource;

#[derive(Debug)]
pub struct Meta {
    size: (f32, f32),
    initial_width: f32,
}

#[cfg(test)]
mod tests {
    use super::TextLayout;
    use crate::generated::{UpdateSceneMsg, StyleProp, Text, Color, TextAlign};

    #[test]
    fn test_new() {
        let mut text_layout = TextLayout::new();

        text_layout.update_scene(&[
            UpdateSceneMsg::SetStyleProp {
                surface: 1,
                prop: StyleProp::Text(Some(Text {
                    color: Color::black(),
                    align: TextAlign::Left,
                    font_size: 16.,
                    line_height: 16.,
                    text: "Hello".into()
                }))
            }
        ])
    }
}
