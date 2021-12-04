use super::{atlas::Atlas, Font, Glyph, TexData, UvRect, Vec2, AABB, SANS_SERIF_FONT};
use fnv::FnvHashMap;
use std::hash::Hash;

pub struct CachedGlyph {
    pub rect: AABB,
    pub uv: UvRect,
}

impl CachedGlyph {
    pub const EMPTY: Self = Self {
        rect: AABB::ZERO,
        uv: UvRect::MAX,
    };
}

pub struct GlyphCache {
    glyphs: FnvHashMap<CacheKey, CachedGlyph>,
    pub(crate) atlas: Atlas,
}

impl GlyphCache {
    pub fn new() -> Self {
        Self {
            glyphs: FnvHashMap::default(),
            atlas: Atlas::new(1024, 1024),
        }
    }

    pub fn use_glyph(&mut self, glyph: Glyph) -> &CachedGlyph {
        let Self { atlas, glyphs } = self;

        glyphs.entry(CacheKey::new(&glyph)).or_insert_with(|| {
            // TODO: evicting, rebuilding
            if let Some(g) = SANS_SERIF_FONT.outline_glyph(glyph) {
                let pxb = g.px_bounds();
                let rect = AABB::new(Vec2::new(pxb.min.x, pxb.min.y), Vec2::new(pxb.max.x, pxb.max.y));

                if let Some(uv) = atlas.push(pxb.width() as _, pxb.height() as _, |dest: &mut TexData, x1, y1| {
                    g.draw(|x2, y2, a| {
                        let a = (a * 255.) as _;
                        dest[(x1 + x2 as usize, y1 + y2 as usize)] = [a, a, a, a];
                    })
                }) {
                    return CachedGlyph { rect, uv };
                }
            }

            CachedGlyph::EMPTY
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
