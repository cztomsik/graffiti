use super::{LaidGlyph, LaidText, TextLayoutAlgo};
use crate::api::{Text, TextAlign};
use pango::prelude::*;
use pango::{Alignment, WrapMode};
use pangocairo::FontMap;
use pango_sys::*;

pub struct PangoService {
    pango_context: pango::Context,
}

impl PangoService {
    pub fn new() -> Self {
        let font_map = FontMap::new().expect("couldn't get fontmap");
        let pango_context = pango::Context::new();
        pango_context.set_font_map(&font_map);

        PangoService { pango_context }
    }

    fn get_layout(&self, text: &Text) -> pango::Layout {
        let mut description = pango::FontDescription::new();
        description.set_family("Arial");
        description.set_absolute_size(to_scale_f64(text.font_size));

        let layout = pango::Layout::new(&self.pango_context);
        layout.set_font_description(&description);
        layout.set_wrap(WrapMode::Word);
        layout.set_text(&text.text);
        layout.set_alignment(text.align.clone().into());

        layout
    }
}

impl TextLayoutAlgo for PangoService {
    fn layout_text(&mut self, text: &Text, max_width: Option<f32>) -> LaidText {
        let layout = self.get_layout(text);
        layout.set_width(to_scale(max_width.unwrap_or(-1.)));

        let lines = layout.get_line_count();
        let cap_height = from_scale(layout.get_line(0).map(|l| l.get_extents().0.height).unwrap_or(0));
        let baseline = cap_height + ((text.line_height - cap_height) / 2.);
        let mut layout_iter = layout.get_iter().expect("couldnt get LayoutIter");
        let mut glyphs = vec![];

        // It's ugly, but to my extent the only way to get `x`, `glyph_index` and `line_index`
        // so we can do proper line-height text layout
        // I've tried many times and it's unlikely that there is better way
        for line_i in 0..lines {
            if let Some(run) = layout_iter.get_run_readonly() {
                unsafe {
                    let (_, run): (usize, &PangoGlyphItem) = std::mem::transmute(run);

                    for i in 0..(*run.glyphs).num_glyphs {
                        let glyph_index = (*(*(*run).glyphs).glyphs.offset(i as isize)).glyph;
                        let extents = layout_iter.get_char_extents();

                        glyphs.push(LaidGlyph {
                            glyph_index,
                            x: from_scale(extents.x),
                            y: (line_i as f32 * text.line_height) + baseline
                        });

                        layout_iter.next_char();
                    }
                }
            }

            if !layout_iter.next_char() {
                break;
            }
        }

        LaidText {
            lines,
            width: layout.get_pixel_size().0 as f32,
            glyphs,
        }
    }
}

// pango values are scaled
fn from_scale(v: i32) -> f32 {
    (v as f32) / (pango::SCALE as f32)
}

fn to_scale(v: f32) -> i32 {
    (v * (pango::SCALE as f32)) as i32
}

fn to_scale_f64(v: f32) -> f64 {
    (v as f64) * (pango::SCALE as f64)
}

impl Into<Alignment> for TextAlign {
    fn into(self) -> Alignment {
        match self {
            TextAlign::Left => Alignment::Left,
            TextAlign::Center => Alignment::Center,
            TextAlign::Right => Alignment::Right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::Color;

    #[test]
    fn test() {
        let mut svc = PangoService::new();

        let text = Text {
            color: Color(0, 0, 0, 1),
            font_size: 24.,
            line_height: 30.,
            align: TextAlign::Left,
            text: "Hello world\n\nHello".into()
        };

        let res = svc.layout_text(&text, Some(100.));

        println!("layout_text {:#?}", &res);

        assert_eq!(res.lines, 3);

        let res = res.glyphs;

        assert_eq!(res.len(), 16);
        assert_eq!(res[0].x, 0.);
        assert_eq!(res[0].y, 0.);

        assert_eq!(res[6].x, 0.);
        assert_ne!(res[6].y, 0.);

        assert_ne!(res[15].x, 0.);
        assert_ne!(res[15].y, 0.);
    }
}
