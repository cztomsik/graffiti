use super::{Atlas, Font, Glyph, TexData, Vec2, AABB, SANS_SERIF_FONT};
use std::collections::HashMap;
use std::hash::Hash;

pub struct CachedGlyph {
    pub rect: AABB,
    pub uv: AABB,
}

pub struct GlyphCache {
    glyphs: HashMap<CacheKey, CachedGlyph>,
    pub(crate) atlas: Atlas,
}

impl GlyphCache {
    pub fn new() -> Self {
        let mut atlas = Atlas::new(1024, 1024);

        // temp hack for solid_rects (which have uv::ZERO)
        // TODO: use negative xy or uv or multi-uv or something better
        atlas.push(1, 1, |tex, _, _| tex[(0, 0)] = [255, 255, 255, 255]);

        Self {
            glyphs: HashMap::new(),
            atlas,
        }
    }

    pub fn use_glyph(&mut self, glyph: Glyph) -> &CachedGlyph {
        let Self { atlas, glyphs } = self;

        glyphs.entry(CacheKey::new(&glyph)).or_insert_with(|| {
            // TODO: evicting, rebuilding
            if let Some(g) = SANS_SERIF_FONT.outline_glyph(glyph) {
                let pxb = g.px_bounds();
                let rect = AABB::new(Vec2::new(pxb.min.x, pxb.min.y), Vec2::new(pxb.max.x, pxb.max.y));

                let uv = atlas
                    .push(pxb.width() as _, pxb.height() as _, |dest, x1, y1| {
                        g.draw(|x2, y2, a| {
                            let a = (a * 255.) as _;
                            dest[(x1 + x2 as usize, y1 + y2 as usize)] = [a, a, a, a];
                        })
                    })
                    .unwrap_or(AABB::ZERO);

                return CachedGlyph { rect, uv };
            }

            // unlikely, layout should skip empty & unknown glyphs
            CachedGlyph {
                rect: AABB::ZERO,
                uv: AABB::ZERO,
            }
        })
    }

    pub fn tex_data(&self) -> &TexData {
        self.atlas.tex_data()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct CacheKey(u16, u32);

impl CacheKey {
    pub fn new(glyph: &Glyph) -> Self {
        Self(glyph.id.0, glyph.scale.x.to_bits())
    }
}
