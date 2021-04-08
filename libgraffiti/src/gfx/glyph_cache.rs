use super::{Atlas, Font, Glyph, TexData, Vec2, AABB, SANS_SERIF_FONT};

pub struct CachedGlyph {
    pub rect: AABB,
    pub uv: AABB,
}

pub struct GlyphCache {
    // TODO: HashMap<Key, CachedGlyph>
    atlas: Atlas,
}

impl GlyphCache {
    pub fn new() -> Self {
        Self { atlas: Atlas::new() }
    }

    pub fn use_glyph(&mut self, glyph: Glyph) -> CachedGlyph {
        // TODO: actual caching/atlasing
        if let Some(g) = SANS_SERIF_FONT.outline_glyph(glyph) {
            let rect = g.px_bounds();
            let rect = AABB::new(Vec2::new(rect.min.x, rect.min.y), Vec2::new(rect.max.x, rect.max.y));

            return CachedGlyph { rect, uv: AABB::ZERO };
        }

        // unlikely, layout should skip empty & unknown glyphs
        CachedGlyph {
            rect: AABB::ZERO,
            uv: AABB::ZERO,
        }
    }

    pub fn tex_data(&self) -> &TexData {
        self.atlas.tex_data()
    }
}
