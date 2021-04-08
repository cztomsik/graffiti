use super::{Atlas, Font, Glyph, TexData, Vec2, AABB, SANS_SERIF_FONT};
use std::collections::HashMap;
use std::hash::Hash;

pub struct CachedGlyph {
    pub rect: AABB,
    pub uv: AABB,
}

pub struct GlyphCache {
    glyphs: HashMap<CacheKey, CachedGlyph>,
    atlas: Atlas,
}

impl GlyphCache {
    pub fn new() -> Self {
        Self {
            glyphs: HashMap::new(),
            atlas: Atlas::new(),
        }
    }

    pub fn use_glyph(&mut self, glyph: Glyph) -> &CachedGlyph {
        self.glyphs.entry(CacheKey::new(&glyph)).or_insert_with(move || {
            // TODO: evicting
            if let Some(g) = SANS_SERIF_FONT.outline_glyph(glyph) {
                let rect = g.px_bounds();
                let rect = AABB::new(Vec2::new(rect.min.x, rect.min.y), Vec2::new(rect.max.x, rect.max.y));
                let uv = AABB::ZERO;

                g.draw(|x, y, a| {
                    println!("TODO: atlas draw {:?}", (x, y, a));
                });

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
