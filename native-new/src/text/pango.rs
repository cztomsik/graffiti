use super::{TextMeasurer, TextShaper};
use crate::api::{Text, TextAlign};
use pango::prelude::*;
use pango::{WrapMode, Alignment, GlyphItem};
use pangocairo::FontMap;
use crate::text::LaidGlyph;
use std::ffi::c_void;
use std::os::raw::c_int;

pub struct PangoService {
    pango_context: pango::Context
}

impl PangoService {
    pub fn new() -> Self {
        let font_map = FontMap::new().expect("couldn't get fontmap");
        let pango_context = pango::Context::new();
        pango_context.set_font_map(&font_map);

        PangoService {
            pango_context
        }
    }

    fn get_layout(&self, text: &Text) -> pango::Layout {
        let mut description = pango::FontDescription::new();
        description.set_family("Arial");
        description.set_size(to_scale(text.font_size));

        let layout = pango::Layout::new(&self.pango_context);
        layout.set_font_description(&description);
        layout.set_wrap(WrapMode::Word);
        layout.set_text(&text.text);
        layout.set_alignment(text.align.clone().into());
        //layout.set_spacing(to_scale((text.line_height - text.font_size) / 2.));

        layout
    }
}

impl TextMeasurer for PangoService {
    fn measure_text(&self, text: &Text, avail_width: f32) -> (f32, f32) {
        let layout = self.get_layout(text);
        layout.set_width(to_scale(avail_width));

        let (extents, _) = layout.get_extents();

        (from_scale(extents.width), layout.get_line_count() as f32 * text.line_height)
    }
}

impl TextShaper for PangoService {
    fn shape_text(&self, text: &Text, size: (f32, f32)) -> Vec<LaidGlyph> {
        let layout = self.get_layout(text);
        layout.set_width(to_scale(size.0));
        //layout.set_height(to_scale(size.1));

        let mut laid_glyphs = Vec::new();
        let mut layout_iter = layout.get_iter().expect("couldn't get LayoutIter");

        loop {
            match layout_iter.get_run_readonly() {
                Some(run) => {
                    unsafe {
                        let run = get_ffi_run(run).glyphs;

                        for i in 0..(*run).num_glyphs {
                            let extents = layout_iter.get_char_extents();
                            let info = (*run).glyphs.offset(i as isize);

                            laid_glyphs.push(LaidGlyph {
                                glyph_index: (*info).glyph,
                                x: from_scale(extents.x + (*info).geometry.x_offset),
                                y: from_scale(extents.y + (*info).geometry.y_offset)
                            });

                            layout_iter.next_char();
                        }
                    }

                },
                None => {}
            };

            if ! layout_iter.next_run() {
                break;
            }
        }

        laid_glyphs
    }
}

// pango values are scaled
fn from_scale(v: i32) -> f32 {
    (v as f32) / (pango::SCALE as f32)
}

fn to_scale(v: f32) -> i32 {
    (v * (pango::SCALE as f32)) as i32
}

unsafe fn get_ffi_run(run: GlyphItem) -> &'static PangoGlyphItem {
    let (_, run): (usize, &PangoGlyphItem) = std::mem::transmute(run);

    run
}

#[repr(C)]
pub struct PangoGlyphItem {
    pub item: *mut c_void,
    pub glyphs: *mut PangoGlyphString,
}

#[repr(C)]
pub struct PangoGlyphString {
    pub num_glyphs: c_int,
    pub glyphs: *mut PangoGlyphInfo,
    pub log_clusters: *mut c_int,
    pub space: c_int,
}

#[repr(C)]
pub struct PangoGlyphInfo {
    pub glyph: u32,
    pub geometry: PangoGlyphGeometry,
    pub attr: u32,
}

#[repr(C)]
pub struct PangoGlyphGeometry {
    pub width: i32,
    pub x_offset: i32,
    pub y_offset: i32,
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
        let svc = PangoService::new();

        let text = Text {
            color: Color(0, 0, 0, 1),
            font_size: 24.,
            line_height: 30.,
            align: TextAlign::Left,
            text: "Hello world\n\nHello".into()
        };

        let measure = svc.measure_text(&text, 100.);
        println!("measure {:#?}", &measure);


        let res = svc.shape_text(&text, (100., 150.));
        println!("{:#?}", &res);

        assert_eq!(res.len(), 20);
        assert_eq!(res[0].x, 0.);
        assert_eq!(res[0].y, 0.);

    }
}
