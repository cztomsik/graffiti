// x don't hide hard-limitations behind leaky abstractions
//   (textures, rects only)
//
// x provide way to render:
//   - transform, clip, opacity
//   x outline shadow (+ radius)
//     x gen image elsewhere
//   x outline
//   x bg color
//   x image
//   x linear/radial gradient
//     x gen image elsewhere
//   x inset shadow (+ radius)
//     x gen image elsewhere
//   x text
//     x cached-layer of msdf quads
//
//   - pseudo text shadow
//     - draw same text many times
//     - or just do it properly with filters (TODO)
//
//   x border
//     x solid
//     - triangle (half of the edge is transparent)
//       - maybe (solid only) push_triangle but supporting arbitrary fill styles could
//         be very challenging for other backends
//         - raqote can't do perspective 3D transform needed for 3 different uv coords.
//     x inset/outset
//     x radius corner (solid/dotted/dashed/inset/outset/ridge)
//       - gen image/msdf texture elsewhere
//         (for uniform edges, one should be fine for each style)
//
//   - filter, backdrop-filter (postprocess)
//
// - it should be fast to change text color
//   - not sure yet, maybe shared uniform for color multiplying
//     (and opacity could be just special-case of that)

use super::Color;
use crate::commons::{Bounds, Pos};

// ref impl.
#[cfg(feature = "raqote")]
pub mod raqote;

// gl impl
pub mod gl;

// - can fill rects/triangles using specific graphics API like OpenGL
// - simple, can't do high-level optimizations
// - work has to be prepared first, by creating & building a layer
// - layer is an abstract container holding future render operations,
//   useful for something which doesn't change too much
//   so it's possible to cache & compose efficiently
// - layers can reference each other (and can be changed afterwards)
pub trait RenderBackend: Sized {
    // impl-specific handles
    type LayerId: Copy;
    type TextureId: Copy;

    // impl-specific layer-state builder
    type LayerBuilder: LayerBuilder<Self>;

    // viewport resize
    fn resize(&mut self, width: f32, height: f32);

    // layer is empty until initialized with builder
    // so it's possible to create one even when there's nothing to draw yet
    fn create_layer(&mut self) -> Self::LayerId;

    // replace layer "contents" (may affect referencing layers)
    fn rebuild_layer_with(&mut self, layer: Self::LayerId, f: impl FnMut(&mut Self::LayerBuilder));

    // actually draw something
    fn render_layer(&mut self, layer: Self::LayerId);

    // so there's no copying
    fn create_texture(&mut self, width: i32, height: i32, data: Box<[u8]>) -> Self::TextureId;

    // needed for atlasing
    fn update_texture(&mut self, texture: Self::TextureId, f: impl FnMut(&mut [u8]));
}

pub trait LayerBuilder<RB: RenderBackend> {
    // TODO: push/pop transform/clip/opacity

    fn push_rect(&mut self, bounds: Bounds, style: FillStyle<RB>);

    fn push_layer(&mut self, layer: RB::LayerId, origin: Pos);
}

#[derive(Debug, Clone, Copy)]
pub enum FillStyle<RB: RenderBackend> {
    // not sure if Msdf can draw sharp triangles/rects (bcof. antialiasing)
    // but this is also more convenient (no need to prepare textures, etc.)
    // so it's probably a good idea to keep it anyway
    SolidColor(Color),

    // images, gradients, shadows, ...
    Texture(RB::TextureId, Bounds),

    // text, radii corners, maybe even paths & preprocessed SVG (later)
    Msdf {
        texture: RB::TextureId,
        uv: Bounds,
        factor: f32,
        color: Color,
    },
}
