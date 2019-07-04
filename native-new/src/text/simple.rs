use crate::text::{TextLayout, Glyph};
use font_kit::source::SystemSource;
use crate::SceneListener;
use crate::generated::{UpdateSceneMsg, StyleProp, SurfaceId, Text};
use std::collections::BTreeMap;
use crate::helpers::Storage;

pub struct SimpleTextLayout {
    font: font_kit::font::Font,
    metas: BTreeMap<SurfaceId, Meta>,
    glyphs: BTreeMap<SurfaceId, Vec<Glyph>>
}

#[derive(Debug)]
pub struct Meta {
    size: (f32, f32),
    initial_width: f32,
}

impl SceneListener for SimpleTextLayout {
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
}

impl TextLayout for SimpleTextLayout {
    fn wrap(&mut self, surface: SurfaceId, max_width: Option<f32>) {
        // TODO
    }

    fn get_size(&self, surface: SurfaceId) -> (f32, f32) {
        self.metas.get(&surface).expect("not a text").size
    }

    fn get_glyphs(&self, surface: SurfaceId) -> &[Glyph] {
        self.glyphs.get(&surface).expect("not a text")
    }
}

impl SimpleTextLayout {
    pub fn new() -> Self {
        let font = SystemSource::new()
            .select_by_postscript_name("ArialMT")
            .unwrap()
            .load()
            .unwrap();

        SimpleTextLayout {
            font,
            metas: BTreeMap::new(),
            glyphs: BTreeMap::new()
        }
    }

    fn layout_text(&self, text: &Text) -> (Meta, Vec<Glyph>) {
        let mut size = (0., text.line_height);
        let mut x = 0.;
        let mut y = 0.;

        let glyphs = text.text.chars().into_iter().map(|c| {
            let glyph_id = self.font.glyph_for_char(c).unwrap_or(0);
            let advance = match self.font.advance(glyph_id) {
                Ok(v) => (v.x, v.y),
                Err(e) => (0., 0.)
            };

            let glyph = Glyph {
                glyph_id,
                x,
                y,
            };

            x += advance.0;
            y += advance.1;

            glyph
        }).collect();

        let meta = Meta {
            size,
            initial_width: size.0
        };

        debug!("{:#?} {:#?}", &meta, &glyphs);

        (meta, glyphs)
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleTextLayout;
    use crate::SceneListener;
    use crate::generated::{UpdateSceneMsg, StyleProp, Text, Color, TextAlign};

    #[test]
    fn test_new() {
        let mut text_layout = SimpleTextLayout::new();

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
