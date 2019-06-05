/// Note that webrender API is not stable and we need to catch up often so this
/// won't ever be perfect.

use crate::generated::{Vector2f, TextAlign, Color, BorderRadius, BorderSide, BorderStyle, SurfaceId};
//use crate::text::{LaidGlyph, LaidText};
use gleam::gl::Gl;
use image;
use image::GenericImageView;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver, Sender};
use webrender::api::{
    AddImage, AlphaType, BorderDetails, BorderDisplayItem, BorderRadius as WRBorderRadius,
    BorderSide as WRBorderSide, BorderStyle as WRBorderStyle, BoxShadowClipMode,
    BoxShadowDisplayItem, ColorF, ColorU, DisplayListBuilder, DocumentId, Epoch,
    FontInstanceKey, GlyphInstance, ImageData, ImageDescriptor,
    ImageDisplayItem, ImageFormat, ImageRendering, CommonItemProperties,
    NormalBorder, PipelineId, RectangleDisplayItem, RenderApi, ClipId,
    ResourceUpdate, SpaceAndClipInfo, DisplayItem, TextDisplayItem, Transaction,
    HitTestFlags, ComplexClipRegion, ClipMode, ScrollLocation, RenderNotifier, ScrollSensitivity, ExternalScrollId,
    units::{LayoutPoint, LayoutSize, LayoutVector2D, WorldPoint, LayoutRect, DeviceIntSize}
};
use webrender::euclid::{TypedSideOffsets2D, TypedSize2D, TypedVector2D};
use webrender::{Renderer, RendererOptions};

pub struct WebrenderRenderer {
    renderer: Renderer,
    render_api: RenderApi,
    document_id: DocumentId,
    rx: Receiver<()>,

    device_size: DeviceIntSize,
    // so that we can reuse already uploaded images
    // this can be (periodically) cleaned up by simply going through all keys and
    // looking what has (not) been used in the last render (and can be evicted)
    // _uploaded_images: BTreeMap<String, ImageKey>
}

impl WebrenderRenderer {
    pub fn new(gl: Rc<Gl>, device_size: (i32, i32)) -> Self {
        let device_size = DeviceIntSize::new(device_size.0, device_size.1);
        let (renderer, mut render_api, rx) = Self::init_webrender(gl, device_size);
        let document_id = render_api.add_document(device_size, 0);

        Self::load_fonts(&mut render_api, document_id, &rx);

        WebrenderRenderer {
            renderer,
            render_api,
            document_id,
            rx,

            device_size,
        }
    }

    fn init_webrender(gl: Rc<Gl>, start_size: DeviceIntSize) -> (Renderer, RenderApi, Receiver<()>) {
        // so that we can block until the frame is actually rendered
        let (tx, rx) = channel();

        let (renderer, sender) = Renderer::new(
            gl,
            Box::new(SyncNotifier(tx)),
            RendererOptions {
                device_pixel_ratio: 1.0,
                ..RendererOptions::default()
            },
            None,
            start_size
        )
        .expect("couldn't init webrender");
        let render_api = sender.create_api();

        (renderer, render_api, rx)
    }

    fn load_fonts(render_api: &mut RenderApi, document_id: DocumentId, rx: &Receiver<()>) {
        let property = font_loader::system_fonts::FontPropertyBuilder::new()
            .family("Arial")
            .build();
        let (font, font_index) = font_loader::system_fonts::get(&property).unwrap();
        let font_key = render_api.generate_font_key();

        let mut tx = Transaction::new();
        tx.set_root_pipeline(PipelineId::dummy());
        tx.add_raw_font(font_key, font, font_index as u32);

        // TODO: support any size
        for font_size in [10, 12, 14, 16, 20, 24, 34, 40, 48, 60, 96].iter() {
            tx.add_font_instance(
                FontInstanceKey(font_key.0, *font_size),
                font_key,
                app_units::Au::from_px(*font_size as i32),
                None,
                None,
                Vec::new(),
            );
        }

        tx.generate_frame();
        render_api.send_transaction(document_id, tx);
        rx.recv().ok();
    }

    fn send_frame(&mut self, builder: DisplayListBuilder, viewport_size: LayoutSize) {
        let mut tx = Transaction::new();

        // according to https://github.com/servo/webrender/wiki/Path-to-the-Screen
        tx.set_root_pipeline(PIPELINE_ID);
        tx.set_display_list(Epoch(0), None, viewport_size, builder.finalize(), true);
        tx.generate_frame();

        self.send_tx(tx);
    }

    fn send_tx(&mut self, tx: Transaction) {
        self.render_api.send_transaction(self.document_id, tx);
        self.wait_for_frame();
    }

    // this needs rework (rendering should be in its own thread anyway) but it's good enough for now
    fn wait_for_frame(&mut self) {
        self.rx.recv().ok();

        self.renderer.update();
        self.renderer.render(self.device_size).ok();
    }

    pub fn resize(&mut self, device_size: (i32, i32), dpi: f32) {
        self.device_size = DeviceIntSize::new(device_size.0, device_size.1);
        self.render_api.set_document_view(self.document_id, self.device_size.into(), dpi);
    }
}

impl crate::render::Renderer for WebrenderRenderer {
    // not complete (border-radius) but it might be fine for some time
    fn hit_test(&self, (x, y): (f32, f32)) -> SurfaceId {
        let res = self.render_api.hit_test(self.document_id, Some(PIPELINE_ID), WorldPoint::new(x, y), HitTestFlags::empty());

        res.items.get(0).map(|item| item.tag.1 as usize).unwrap_or(0)
    }

    fn scroll(&mut self, (x, y): (f32, f32), delta: (f32, f32)) {
        let mut tx = Transaction::new();
        let scroll_location = ScrollLocation::Delta(LayoutVector2D::new(delta.0 * SCROLL_FACTOR, delta.1 * SCROLL_FACTOR));
        let cursor = WorldPoint::new(x, y);

        tx.scroll(scroll_location, cursor);
        tx.generate_frame();

        self.send_tx(tx);
    }
}

/*
impl SceneRenderer for WebrenderRenderer{
    fn render(&mut self, scene: &dyn Scene) {
        //debug!("render\n{:#?}", surface);

        let surface = 0;
        let Rect(_, _, width, height) = scene.computed_layout(surface);
        let content_size = LayoutSize::new(width, height);
        let pipeline_id = PIPELINE_ID;

        let builder = {
            let mut context = RenderContext {
                scene,
                render_api: &mut self.render_api,

                builder: DisplayListBuilder::with_capacity(
                    pipeline_id,
                    content_size.clone(),
                    BUILDER_CAPACITY,
                ),
                border_radius: WRBorderRadius::zero(),
                layout: CommonItemProperties::new(content_size.into(), SpaceAndClipInfo::root_scroll(PIPELINE_ID))
            };

            context.render_surface(surface);

            context.builder
        };

        self.send_frame(builder, content_size);
    }
}

struct RenderContext<'a> {
    scene: &'a dyn Scene,
    render_api: &'a mut RenderApi,

    builder: DisplayListBuilder,
    border_radius: WRBorderRadius,
    layout: CommonItemProperties
}

impl<'a> RenderContext<'a> {
    fn render_surface(&mut self, surface: SurfaceId) {
        let parent_layout = self.layout;
        let parent_space_and_clip = SpaceAndClipInfo { spatial_id: parent_layout.spatial_id, clip_id: parent_layout.clip_id };

        let Rect(x, y, width, height) = self.scene.computed_layout(surface);

        self.layout = CommonItemProperties::new(
            LayoutRect::new(LayoutPoint::new(x, y), LayoutSize::new(width, height))
                .translate(&parent_layout.clip_rect.origin.to_vector()),
            parent_space_and_clip
        );

        // everything will receive events (important for onMouseLeave)
        self.layout.hit_info = Some((0, surface as u16));

        debug!("surface {} {:?}", surface, self.layout.clip_rect);

        // shared, not directly rendered
        if let Some(border_radius) = self.scene.border_radius(surface) {
            self.border_radius = border_radius.clone().into();

            let clip_region = ComplexClipRegion::new(self.layout.clip_rect.clone(), self.border_radius, ClipMode::Clip);
            let clip_id = self.builder.define_clip(&parent_space_and_clip, self.layout.clip_rect, vec![clip_region], None);

            self.layout.clip_id = clip_id;
        } else {
            self.border_radius = WRBorderRadius::zero();
        }

        // TODO: (outset) shadow shouldn't have tag (& receive events)
        // note that we are using parent clip (shadow should be clipped by parent not by us)
        if let Some(box_shadow) = self.scene.box_shadow(surface) {
            self.builder.push_item(
                &self.box_shadow(box_shadow, parent_space_and_clip.clip_id)
            );
        }

        if let Some(color) = self.scene.background_color(surface) {
            self.push(self.background_color(color.clone()));
        }

        if let Some(image) = self.scene.image(surface) {
            self.push(self.image(image.clone()));
        }

        // TODO: selections (should be below text)
        // (or it could be just overlay with inverse color "effect")

        if let Some(text) = self.scene.text(surface) {
            let (item, glyphs) = self.text(text.clone(), self.scene.text_layout(surface));

            // webrender has a limit on how long the text item can be
            // TODO: use the const from webrender (couldn't find it quickly)
            for glyphs in glyphs.chunks(2000) {
                self.push(item.clone());
                self.builder.push_iter(glyphs);
            }
        }

        if let Some(border) = self.scene.border(surface) {
            self.push(self.border(border.clone()));
            // TODO: children should be in (possibly rounded) clip too so they can't overdraw border (or padding)
        }

        if let Some((width, height)) = self.scene.scroll_frame(surface) {
            debug!("scroll_frame {:?}", (&width, &height, &self.layout));

            let area_rect = LayoutRect::new(self.layout.clip_rect.origin.clone(), LayoutSize::new(width, height));

            let space_and_clip = self.builder.define_scroll_frame(
                &SpaceAndClipInfo { clip_id: self.layout.clip_id, spatial_id: self.layout.spatial_id },
                Some(ExternalScrollId(surface as u64, PIPELINE_ID)),
                area_rect,
                self.layout.clip_rect,
                vec![],
                None,
                ScrollSensitivity::ScriptAndInputEvents,
                LayoutVector2D::zero()
            );
            let hit_info = self.layout.hit_info;
            self.layout = CommonItemProperties::new(area_rect, space_and_clip);
            self.layout.hit_info = hit_info;

            // we need to push something which will receive hit-test events for the whole "area"
            // otherwise scroll would not work in "empty" spaces
            // TODO: stacking context would be probably better
            self.builder.push_item(&self.background_color(Color(0, 0, 0, 0)));
        }

        // children has to be "on top" because of hitbox testing
        for child_surface in self.scene.children(surface) {
            self.render_surface(*child_surface);
        }

        // restore layout
        self.layout = parent_layout;
    }

    fn box_shadow(&self, box_shadow: &BoxShadow, parent_clip_id: ClipId) -> DisplayItem {
        let Vector2f(x, y) = box_shadow.offset;
        let size = box_shadow.spread + box_shadow.blur;

        DisplayItem::BoxShadow(BoxShadowDisplayItem {
            common: CommonItemProperties {
                clip_rect:
                    self.layout
                        .clip_rect
                        .translate(&TypedVector2D::new(x, y))
                        .inflate(size, size),
                clip_id: parent_clip_id,
                ..self.layout
            },
            color: box_shadow.color.clone().into(),

            box_bounds: self.layout.clip_rect,
            offset: box_shadow.offset.clone().into(),
            blur_radius: box_shadow.blur,
            spread_radius: box_shadow.spread,
            border_radius: self.border_radius.clone().into(),

            // TODO: Inset/Outset (outset needs bigger clip-rect)
            clip_mode: BoxShadowClipMode::Outset,
        })
    }

    fn background_color(&self, color: Color) -> DisplayItem {
        DisplayItem::Rectangle(RectangleDisplayItem {
            common: self.layout,
            color: color.into(),
        })
    }

    // TODO: refactor, cache, free + hook to make loading possible from node.js (http)
    fn image(&self, image: Image) -> DisplayItem {
        let image_key = {
            let mut f = File::open(image.url).expect("couldn't open file");
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).expect("couldn't read");

            let image = image::load_from_memory(&buffer).expect("couldn't load image");
            let descriptor = ImageDescriptor::new(
                image.width() as i32,
                image.height() as i32,
                ImageFormat::RGBA8,
                true,
                false,
            );

            let key = self.render_api.generate_image_key();

            self.render_api
                .update_resources(vec![ResourceUpdate::AddImage(AddImage {
                    key,
                    descriptor,
                    data: ImageData::new(
                        image
                            .as_rgba8()
                            .expect("couldn't convert to rgba8")
                            .to_vec(),
                    ),
                    tiling: None,
                })]);

            key
        };

        DisplayItem::Image(ImageDisplayItem {
            common: self.layout,
            bounds: self.layout.clip_rect,
            image_key,
            stretch_size: self.layout.clip_rect.size.into(),
            tile_spacing: TypedSize2D::zero(),
            image_rendering: ImageRendering::Auto,
            alpha_type: AlphaType::PremultipliedAlpha,
            color: ColorF::WHITE,
        })
    }

    // TODO: clip should be enough big to contain `y` and similar characters
    fn text(&self, text: Text, laid_text: LaidText) -> (DisplayItem, Vec<GlyphInstance>) {
        let [mut text_x, text_y] = self.layout.clip_rect.origin.to_array();
        // TODO: text-right

        if let TextAlign::Center = text.align {
            text_x = text_x + (self.layout.clip_rect.size.width - laid_text.width) / 2.;
        }

        let glyphs = laid_text.glyphs
            .iter()
            .map(|LaidGlyph { glyph_index, x, y }| GlyphInstance {
                index: *glyph_index,
                point: LayoutPoint::new(text_x + x, text_y + y),
            })
            .collect();

        let font_key = FontInstanceKey::new(self.render_api.get_namespace_id(), text.font_size as u32);

        let item = DisplayItem::Text(TextDisplayItem {
            common: self.layout,
            bounds: self.layout.clip_rect,
            font_key,
            color: text.color.clone().into(),
            glyph_options: None,
        });

        (item, glyphs)
    }

    fn border(&self, border: Border) -> DisplayItem {
        DisplayItem::Border(BorderDisplayItem {
            common: self.layout,
            bounds: self.layout.clip_rect,
            widths: TypedSideOffsets2D::new(
                border.top.width,
                border.right.width,
                border.bottom.width,
                border.left.width,
            ),
            details: BorderDetails::Normal(NormalBorder {
                top: border.top.into(),
                right: border.right.into(),
                bottom: border.bottom.into(),
                left: border.left.into(),
                radius: self.border_radius.clone().into(),
                do_aa: true,
            }),
        })
    }

    fn push(&mut self, item: DisplayItem) {
        debug!("push {:?}", &item);

        self.builder
            .push_item(&item);
    }
}*/

// unlike browser, we are going to have only one pipeline (per window)
static PIPELINE_ID: PipelineId = PipelineId(0, 0);

static BUILDER_CAPACITY: usize = 512 * 1024;

// no idea but it's very slow otherwise
static SCROLL_FACTOR: f32 = 5.0;

impl Into<ColorF> for Color {
    fn into(self) -> ColorF {
        let Color(r, g, b, a) = self;
        ColorU::new(r, g, b, a).into()
    }
}

impl Into<LayoutVector2D> for Vector2f {
    fn into(self) -> LayoutVector2D {
        LayoutVector2D::new(self.0, self.1)
    }
}

impl Into<WRBorderRadius> for BorderRadius {
    fn into(self) -> WRBorderRadius {
        WRBorderRadius {
            top_left: LayoutSize::new(self.0, self.0),
            top_right: LayoutSize::new(self.1, self.1),
            bottom_left: LayoutSize::new(self.2, self.2),
            bottom_right: LayoutSize::new(self.3, self.3),
        }
    }
}

impl Into<WRBorderSide> for BorderSide {
    fn into(self) -> WRBorderSide {
        WRBorderSide {
            color: self.color.into(),
            style: self.style.into(),
        }
    }
}

// TODO: more styles
impl Into<WRBorderStyle> for BorderStyle {
    fn into(self) -> WRBorderStyle {
        match self {
            BorderStyle::None => WRBorderStyle::None,
            BorderStyle::Solid => WRBorderStyle::Solid,
        }
    }
}

pub struct SyncNotifier(pub Sender<()>);

impl RenderNotifier for SyncNotifier {
    fn clone(&self) -> Box<RenderNotifier> {
        return Box::new(SyncNotifier(self.0.clone()));
    }

    fn wake_up(&self) {
        self.0.send(()).ok();
    }

    fn new_frame_ready(
        &self,
        _document_id: DocumentId,
        _scrolled: bool,
        _composite_needed: bool,
        _render_time_ns: Option<u64>,
    ) {
        self.wake_up();
    }
}
