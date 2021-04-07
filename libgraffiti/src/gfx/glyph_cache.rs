use super::{Atlas, Vec2, AABB, SANS_SERIF_FONT};
use ab_glyph_rasterizer::{point as ab_point, Point as AbPoint, Rasterizer};
use owned_ttf_parser::OutlineBuilder;
use owned_ttf_parser::{AsFaceRef, GlyphId, Rect};

pub struct GlyphCache {
    pub(crate) atlas: Atlas,
    raster: Rasterizer,
}

impl GlyphCache {
    pub fn new() -> Self {
        Self {
            atlas: Atlas::new(),
            raster: Rasterizer::new(128, 128),
        }
    }

    pub fn use_glyph(&mut self, glyph_id: u16) -> AABB {
        let face = SANS_SERIF_FONT.face.as_face_ref();
        let scale = 16. / face.units_per_em().unwrap() as f32;

        // TODO: actual caching/atlasing

        let mut ctx = RasterCtx::new(&mut self.raster);
        ctx.raster.clear();
        //face.outline_glyph(GlyphId(glyph_id), &mut ctx);
        //ctx.raster.for_each_x();

        match face.glyph_bounding_box(GlyphId(glyph_id)) {
            Some(Rect {
                x_min,
                y_min,
                x_max,
                y_max,
            }) => AABB::new(
                // TODO: find slot
                Vec2::new(x_min as f32 * scale, y_min as f32 * scale),
                Vec2::new(x_max as f32 * scale, y_max as f32 * scale),
            ),
            None => AABB::ZERO,
        }
    }
}

struct RasterCtx<'a> {
    raster: &'a mut Rasterizer,
    p: AbPoint,
    p2: AbPoint,
}

impl RasterCtx<'_> {
    fn new(raster: &mut Rasterizer) -> RasterCtx {
        RasterCtx {
            raster,
            p: ab_point(0., 0.),
            p2: ab_point(0., 0.),
        }
    }
}

impl OutlineBuilder for RasterCtx<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.p2 = self.p;
        self.p = ab_point(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.raster.draw_line(self.p, ab_point(x, y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.raster.draw_quad(self.p, ab_point(x1, y1), ab_point(x, y));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.raster
            .draw_cubic(self.p, ab_point(x1, x2), ab_point(x2, y2), ab_point(x, y));
    }

    fn close(&mut self) {
        self.raster.draw_line(self.p2, self.p);
    }
}
