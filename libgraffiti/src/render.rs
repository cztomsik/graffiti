// x decoupled from layout, styles, ... (commons & utils are allowed)
//   x create resources, return handles
// x types & granularity suited for rendering
// x easy to test (pass vec of bounds)
// x stateful

use crate::commons::{Bounds, Lookup, Mat3, Pos};

// handles
// public but opaque types

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceId(usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImageId(usize);

// re-export value types
pub mod value_types;
use self::value_types::*;

// and backend
pub mod backend;
use self::backend::{FillStyle, LayerBuilder, RenderBackend};

// where:
// - `RB` is `RenderBackend` implementation
// - `BK` is some key to get layout bounds
pub struct Renderer<RB: RenderBackend, BK> {
    backend: RB,
    ui_state: UiState<RB, BK>,
}

impl<RB: RenderBackend, BK: Copy> Renderer<RB, BK> {
    pub fn new(mut backend: RB) -> Self {
        let root_layer = backend.create_layer();

        const SDF_TEXTURE: &[u8; 512 * 512 * 4] = include_bytes!("../resources/sheet0.raw");
        backend.create_texture(512, 512, Box::new(*SDF_TEXTURE));

        Self {
            backend,
            ui_state: UiState::new(root_layer),
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.backend.resize(width, height);
    }

    // surface
    pub fn create_surface(&mut self, bounds_key: BK) -> SurfaceId {
        // TODO: maybe defaults shouldn't be here
        // (accept some ContainerState?)
        // but maybe it's inevitable, yoga has some defaults, this has some too, etc.
        self.ui_state.bounds_keys.push(bounds_key);
        self.ui_state.children.push(Vec::new());
        self.ui_state.overflows.push(Overflow::Visible);
        self.ui_state.opacities.push(1.);
        self.ui_state.border_radii.push(None);
        self.ui_state.outline_shadows.push(Vec::new());
        self.ui_state.outlines.push(None);
        self.ui_state.background_colors.push(Color::TRANSPARENT);
        self.ui_state.background_images.push(Vec::new());
        self.ui_state.inset_shadows.push(Vec::new());
        self.ui_state.colors.push(Color::BLACK);
        self.ui_state.borders.push(None);

        SurfaceId(self.ui_state.background_colors.len() - 1)
    }

    pub fn insert_child(&mut self, surface: SurfaceId, index: usize, child: SurfaceId) {
        self.ui_state.children[surface.0].insert(index, child);
    }

    pub fn remove_child(&mut self, surface: SurfaceId, child: SurfaceId) {
        self.ui_state.children[surface.0].retain(|ch| *ch != child);
    }

    // setters (in order in which they are needed during rendering)

    pub fn set_transform(&mut self, surface: SurfaceId, value: Mat3) {}

    pub fn set_overflow(&mut self, surface: SurfaceId, value: Overflow) {
        self.ui_state.overflows[surface.0] = value;
    }

    pub fn set_opacity(&mut self, surface: SurfaceId, value: f32) {
        self.ui_state.opacities[surface.0] = value;
    }

    pub fn set_border_radius(&mut self, surface: SurfaceId, value: Option<BorderRadius>) {
        self.ui_state.border_radii[surface.0] = value;
    }

    pub fn set_outline_shadows(&mut self, surface: SurfaceId, value: Vec<OutlineShadow>) {
        self.ui_state.outline_shadows[surface.0] = value;
    }

    pub fn set_outline(&mut self, surface: SurfaceId, value: Option<Outline>) {
        self.ui_state.outlines[surface.0] = value;
    }

    pub fn set_background_color(&mut self, surface: SurfaceId, value: Color) {
        self.ui_state.background_colors[surface.0] = value;
    }

    pub fn set_background_images(&mut self, surface: SurfaceId, value: Vec<BackgroundImage>) {
        self.ui_state.background_images[surface.0] = value;
    }

    pub fn set_inset_shadows(&mut self, surface: SurfaceId, value: Vec<InsetShadow>) {
        self.ui_state.inset_shadows[surface.0] = value;
    }

    // TODO: set_text_shadow

    pub fn set_color(&mut self, surface: SurfaceId, value: Color) {
        self.ui_state.colors[surface.0] = value;
    }

    pub fn set_border(&mut self, surface: SurfaceId, value: Option<Border>) {
        self.ui_state.borders[surface.0] = value;
    }

    // image
    pub fn create_image(&mut self, width: i32, height: i32, data: Box<[u8]>) -> ImageId {
        // TODO: put it to some existing/new texture (rect-packing)
        self.ui_state.textures.push(self.backend.create_texture(width, height, data));

        ImageId(self.ui_state.textures.len() - 1)
    }

    pub fn set_image_data(&mut self /* data: rgb &[u8] */) {}

    /*
    // text
    pub fn create_text(&mut self, bounds_key: BK) -> TextId {
        self.ui_state.text_bounds_keys.push(bounds_key);
        self.ui_state.text_layers.push(self.backend.create_layer());

        TextId(self.ui_state.text_layers.len() - 1)
    }

    pub fn set_text_data(&mut self, text: TextId, str: String /* TODO: texture + glyphs */) {
        self.backend.rebuild_layer_with(self.ui_state.text_layers[text.0], |b| {
            let mut x = 0.;

            for _c in str.chars() {
                b.push_rect(
                    Bounds {
                        a: Pos { x, y: 0. },
                        b: Pos { x: x + 12., y: 20. },
                    },
                    // TODO: color should be mixed/overridden during render_surface
                    FillStyle::SolidColor(Color::WHITE),
                );
                x += 15.;
            }
        });
    }
    */

    pub fn render_surface(&mut self, surface: SurfaceId, bounds: &impl Lookup<BK, Bounds>) {
        let layer = self.ui_state.root_layer;
        let current_bounds = bounds.lookup(self.ui_state.bounds_keys[surface.0]);
        let ui_state = &self.ui_state;

        self.backend.rebuild_layer_with(layer, |builder| {
            let mut ctx = RenderContext {
                builder,
                ui_state,
                bounds,
                current_bounds,
            };

            ctx.render_surface(surface);
        });

        self.backend.render_layer(layer);
    }
}

// internal impl starts here

// data-oriented storage
// TODO: BTreeMap, flags, freelists
// TODO: #[derive(Default)] so the `new()` can be removed?
struct UiState<RB: RenderBackend, BK> {
    bounds_keys: Vec<BK>,
    children: Vec<Vec<SurfaceId>>,
    opacities: Vec<f32>,
    border_radii: Vec<Option<BorderRadius>>,
    overflows: Vec<Overflow>,
    outline_shadows: Vec<Vec<OutlineShadow>>,
    outlines: Vec<Option<Outline>>,
    background_colors: Vec<Color>,
    background_images: Vec<Vec<BackgroundImage>>,
    inset_shadows: Vec<Vec<InsetShadow>>,
    colors: Vec<Color>,
    borders: Vec<Option<Border>>,

    text_layers: Vec<RB::LayerId>,

    root_layer: RB::LayerId,
    textures: Vec<RB::TextureId>,
}

impl<RB: RenderBackend, BK: Copy> UiState<RB, BK> {
    fn new(root_layer: RB::LayerId) -> Self {
        Self {
            bounds_keys: Vec::new(),
            children: Vec::new(),
            overflows: Vec::new(),
            opacities: Vec::new(),
            border_radii: Vec::new(),
            outline_shadows: Vec::new(),
            outlines: Vec::new(),
            background_colors: Vec::new(),
            background_images: Vec::new(),
            inset_shadows: Vec::new(),
            colors: Vec::new(),
            borders: Vec::new(),

            text_layers: Vec::new(),

            root_layer,
            textures: Vec::new(),
        }
    }
}

struct RenderContext<'a, RB: RenderBackend, BK: Copy, BS: Lookup<BK, Bounds>> {
    builder: &'a mut RB::LayerBuilder,
    ui_state: &'a UiState<RB, BK>,
    bounds: &'a BS,
    current_bounds: Bounds,
}

impl<RB: RenderBackend, BK: Copy, BS: Lookup<BK, Bounds>> RenderContext<'_, RB, BK, BS> {
    fn render_surface(&mut self, surface: SurfaceId) {
        // TODO: transform
        // TODO: overflow (scroll)
        // TODO: opacity
        // TODO: border_radius (clip downwards, (border/shadow only on this level))

        for s in &self.ui_state.outline_shadows[surface.0] {
            self.render_outline_shadow(s);
        }

        if let Some(o) = &self.ui_state.outlines[surface.0] {
            self.render_outline(o);
        }

        // TODO: clip if Overflow::Hidden
        // (should be after outline)

        self.render_background_color(self.ui_state.background_colors[surface.0]);

        for b in &self.ui_state.background_images[surface.0] {
            self.render_background_image(b);
        }

        for s in &self.ui_state.inset_shadows[surface.0] {
            self.render_inset_shadow(s);
        }

        for ch in &self.ui_state.children[surface.0] {
            let prev_bounds = self.current_bounds;

            self.current_bounds = self.bounds.lookup(self.ui_state.bounds_keys[ch.0]).translate(prev_bounds.a);
            self.render_surface(*ch);

            self.current_bounds = prev_bounds;
        }

        // TODO: render_text

        if let Some(b) = &self.ui_state.borders[surface.0] {
            self.render_border(b);
        }
    }

    fn render_outline_shadow(&mut self, shadow: &OutlineShadow) {
        if shadow.blur != 0. {
            println!("TODO: OutlineShadow blur");
        }

        self.builder
            .push_rect(self.current_bounds.inflate_uniform(shadow.spread), FillStyle::SolidColor(shadow.color));
    }

    fn render_outline(&mut self, outline: &Outline) {
        let Outline { width, color, .. } = *outline;
        let Bounds { a, b } = self.current_bounds;

        // top
        self.builder.push_rect(
            Bounds {
                a,
                b: Pos { x: b.x + width, y: a.y - width },
            },
            FillStyle::SolidColor(color),
        );

        // right
        self.builder.push_rect(
            Bounds {
                a: Pos { x: b.x + width, y: a.y },
                b: Pos { x: b.x, y: b.y + width },
            },
            FillStyle::SolidColor(color),
        );

        // bottom
        self.builder.push_rect(
            Bounds {
                a: Pos { x: a.x - width, y: b.y + width },
                b,
            },
            FillStyle::SolidColor(color),
        );

        // left
        self.builder.push_rect(
            Bounds {
                a: Pos { x: a.x - width, y: a.y - width },
                b: Pos { x: a.x, y: b.y },
            },
            FillStyle::SolidColor(color),
        );
    }

    fn render_background_color(&mut self, color: Color) {
        if color.a != 0 {
            self.builder.push_rect(self.current_bounds, FillStyle::SolidColor(color));
        }
    }

    fn render_background_image(&mut self, background_image: &BackgroundImage) {
        match background_image {
            BackgroundImage::Image { image } => self.builder.push_rect(
                self.current_bounds,
                FillStyle::Texture(self.ui_state.textures[image.0], Bounds { a: Pos::ZERO, b: Pos::ONE }),
            ),
            BackgroundImage::LinearGradient {} => println!("TODO: render linear gradient"),
            BackgroundImage::RadialGradient {} => println!("TODO: render radial gradient"),
        }
    }

    fn render_inset_shadow(&mut self, shadow: &InsetShadow) {
        println!("TODO: render_inset_shadow");
    }

    /*
    fn render_text(&mut self, text: TextId) {
        // TODO: override/mix color
        self.builder.push_layer(
            self.ui_state.text_layers[text.0],
            self.bounds[self.ui_state.bounds_keys[text.0]].a.translate(self.current_bounds.a),
        );
    }
    */

    //fn render_text_shadow(&mut self) {}

    fn render_border(&mut self, border: &Border) {
        // note the border is always inside (it acts like padding in layout)

        // TODO: border_radius

        // TODO: corners (overdraw will be visible with alpha colors)
        // TODO: different edge colors (push_triangle)

        let Bounds { a, b } = self.current_bounds;

        if let Some(BorderSide { width, style, color }) = border.top {
            if style == BorderStyle::Solid {
                self.builder.push_rect(
                    Bounds {
                        a,
                        b: Pos { x: b.x, y: a.y + width },
                    },
                    FillStyle::SolidColor(color),
                )
            }
        }

        if let Some(BorderSide { width, style, color }) = border.right {
            if style == BorderStyle::Solid {
                self.builder.push_rect(
                    Bounds {
                        a: Pos { x: b.x - width, y: a.y },
                        b,
                    },
                    FillStyle::SolidColor(color),
                )
            }
        }

        if let Some(BorderSide { width, style, color }) = border.bottom {
            if style == BorderStyle::Solid {
                self.builder.push_rect(
                    Bounds {
                        a: Pos { x: a.x, y: b.y - width },
                        b,
                    },
                    FillStyle::SolidColor(color),
                )
            }
        }

        if let Some(BorderSide { width, style, color }) = border.left {
            if style == BorderStyle::Solid {
                self.builder.push_rect(
                    Bounds {
                        a,
                        b: Pos { x: a.x + width, y: b.y },
                    },
                    FillStyle::SolidColor(color),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_surface() {
        let mut r = create_test_renderer();
        let c = r.create_surface(0);

        r.render_surface(c, &vec![Bounds::ZERO]);

        assert_eq!(r.backend.log, vec!["create_layer", "rebuild_layer 1", "render_layer 1"]);
    }

    #[test]
    fn outline() {
        let mut r = create_test_renderer();
        let c = r.create_surface(0);

        r.set_outline(
            c,
            Some(Outline {
                width: 1.,
                style: OutlineStyle::Solid,
                color: Color::BLUE,
            }),
        );
        r.render_surface(
            c,
            &vec![Bounds {
                a: Pos::ZERO,
                b: Pos { x: 100., y: 100. },
            }],
        );

        assert_eq!(
            r.backend.log,
            vec![
                "create_layer",
                "rebuild_layer 1",
                "push_rect Bounds((0.0, 0.0), (101.0, -1.0)) SolidColor(#0000ff)",
                "push_rect Bounds((101.0, 0.0), (100.0, 101.0)) SolidColor(#0000ff)",
                "push_rect Bounds((-1.0, 101.0), (100.0, 100.0)) SolidColor(#0000ff)",
                "push_rect Bounds((-1.0, -1.0), (0.0, 100.0)) SolidColor(#0000ff)",
                "render_layer 1"
            ]
        );
    }

    #[test]
    fn background_color() {
        let mut r = create_test_renderer();
        let c = r.create_surface(0);

        r.set_background_color(c, Color::GREEN);
        r.render_surface(
            c,
            &vec![Bounds {
                a: Pos::ZERO,
                b: Pos { x: 100., y: 100. },
            }],
        );

        assert_eq!(
            r.backend.log,
            vec![
                "create_layer",
                "rebuild_layer 1",
                "push_rect Bounds((0.0, 0.0), (100.0, 100.0)) SolidColor(#00ff00)",
                "render_layer 1"
            ]
        );
    }

    #[test]
    fn children() {
        let mut r = create_test_renderer();
        let parent = r.create_surface(0);
        let child = r.create_surface(1);

        r.insert_child(parent, 0, child);

        r.set_background_color(child, Color { r: 255, g: 0, b: 0, a: 255 });

        r.render_surface(
            parent,
            &vec![
                Bounds {
                    a: Pos::ZERO,
                    b: Pos { x: 100., y: 100. },
                },
                Bounds {
                    a: Pos { x: 50., y: 50. },
                    b: Pos { x: 150., y: 150. },
                },
            ],
        );

        assert_eq!(
            r.backend.log,
            vec![
                "create_layer",
                "rebuild_layer 1",
                "push_rect Bounds((50.0, 50.0), (150.0, 150.0)) SolidColor(#ff0000)",
                "render_layer 1"
            ]
        );
    }

    #[test]
    fn it_works() {
        let mut r = create_test_renderer();
        let c = r.create_surface(0);

        r.set_overflow(c, Overflow::Visible);
        r.set_opacity(c, 0.5);
        r.set_border_radius(
            c,
            Some(BorderRadius {
                top_left: 5.,
                top_right: 5.,
                bottom_right: 5.,
                bottom_left: 5.,
            }),
        );
        r.set_outline_shadows(
            c,
            vec![OutlineShadow {
                offset: Pos::ZERO,
                blur: 5.,
                spread: 5.,
                color: Color::BLACK,
            }],
        );
        r.set_outline(
            c,
            Some(Outline {
                width: 1.,
                style: OutlineStyle::Solid,
                color: Color::BLACK,
            }),
        );
        r.set_background_color(c, Color::BLACK);
        r.set_inset_shadows(
            c,
            vec![InsetShadow {
                offset: Pos::ZERO,
                blur: 5.,
                spread: 5.,
                color: Color::BLACK,
            }],
        );
        r.set_color(c, Color::BLACK);
        r.set_border(
            c,
            Some(Border {
                top: Some(BorderSide {
                    width: 1.,
                    style: BorderStyle::Solid,
                    color: Color::RED,
                }),
                right: Some(BorderSide {
                    width: 1.,
                    style: BorderStyle::Solid,
                    color: Color::GREEN,
                }),
                bottom: Some(BorderSide {
                    width: 1.,
                    style: BorderStyle::Solid,
                    color: Color::BLUE,
                }),
                left: Some(BorderSide {
                    width: 1.,
                    style: BorderStyle::Solid,
                    color: Color::YELLOW,
                }),
            }),
        );

        r.render_surface(c, &vec![Bounds::ZERO]);

        assert_eq!(
            r.backend.log,
            vec![
                "create_layer",
                "rebuild_layer 1",
                "push_rect Bounds((-5.0, -5.0), (5.0, 5.0)) SolidColor(#000000)",
                "push_rect Bounds((0.0, 0.0), (1.0, -1.0)) SolidColor(#000000)",
                "push_rect Bounds((1.0, 0.0), (0.0, 1.0)) SolidColor(#000000)",
                "push_rect Bounds((-1.0, 1.0), (0.0, 0.0)) SolidColor(#000000)",
                "push_rect Bounds((-1.0, -1.0), (0.0, 0.0)) SolidColor(#000000)",
                "push_rect Bounds((0.0, 0.0), (0.0, 0.0)) SolidColor(#000000)",
                "push_rect Bounds((0.0, 0.0), (0.0, 1.0)) SolidColor(#ff0000)",
                "push_rect Bounds((-1.0, 0.0), (0.0, 0.0)) SolidColor(#00ff00)",
                "push_rect Bounds((0.0, -1.0), (0.0, 0.0)) SolidColor(#0000ff)",
                "push_rect Bounds((0.0, 0.0), (1.0, 0.0)) SolidColor(#ffff00)",
                "render_layer 1"
            ]
        );
    }

    fn create_test_renderer<BK: Copy>() -> Renderer<TestRenderBackend, BK> {
        Renderer::new(TestRenderBackend { log: Vec::new() })
    }

    #[derive(Debug)]
    struct TestRenderBackend {
        log: Vec<String>,
    }

    impl RenderBackend for TestRenderBackend {
        type LayerId = usize;
        type TextureId = usize;
        type LayerBuilder = Vec<String>;

        fn resize(&mut self, width: f32, height: f32) {
            self.log.push(format!("resize {:?} {:?}", width, height));
        }

        fn create_layer(&mut self) -> Self::LayerId {
            self.log.push("create_layer".to_string());

            self.log.len()
        }

        fn rebuild_layer_with(&mut self, layer: Self::LayerId, mut f: impl FnMut(&mut Self::LayerBuilder)) {
            self.log.push(format!("rebuild_layer {:?}", layer));

            f(&mut self.log);
        }

        fn render_layer(&mut self, layer: Self::LayerId) {
            self.log.push(format!("render_layer {:?}", layer));
        }

        fn create_texture(&mut self, width: i32, height: i32, data: Box<[u8]>) -> Self::TextureId {
            self.log.push(format!("create_texture {:?} {:?}", width, height));

            self.log.len()
        }

        fn update_texture(&mut self, texture: Self::TextureId, f: impl FnMut(&mut [u8])) {
            self.log.push(format!("update_texture {:?}", texture));
        }
    }

    impl LayerBuilder<TestRenderBackend> for Vec<String> {
        fn push_rect(&mut self, bounds: Bounds, style: FillStyle<TestRenderBackend>) {
            self.push(format!("push_rect {:?} {:?}", bounds, style));
        }

        fn push_layer(&mut self, layer: usize, origin: Pos) {
            self.push(format!("push_layer {:?} {:?}", layer, origin));
        }
    }
}
