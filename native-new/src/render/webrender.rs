use super::{
    Border, BorderRadius, BorderSide, BorderStyle, BoxShadow, Color, ComputedLayout, Image,
    RenderService, Text,
};
use crate::generated::Vector2f;
use crate::scene::SurfaceData;
use crate::temp;
use image;
use image::GenericImageView;
use pango::prelude::*;
use pango::WrapMode;
use pangocairo::FontMap;
use std::fs::File;
use std::io::prelude::*;
use webrender::api::{
    AddImage, AlphaType, BorderDetails, BorderDisplayItem, BorderRadius as WRBorderRadius,
    BorderSide as WRBorderSide, BorderStyle as WRBorderStyle, BoxShadowClipMode,
    BoxShadowDisplayItem, ColorF, ColorU, DisplayListBuilder, FontInstanceKey, FontKey,
    GlyphInstance, IdNamespace, ImageData, ImageDescriptor, ImageDisplayItem, ImageFormat,
    ImageRendering, LayoutPoint, LayoutPrimitiveInfo, LayoutRect, LayoutSize, LayoutVector2D,
    NormalBorder, PipelineId, RectangleDisplayItem, ResourceUpdate, SpaceAndClipInfo,
    SpecificDisplayItem, TextDisplayItem,
};
use webrender::euclid::{TypedSideOffsets2D, TypedSize2D, TypedVector2D};

static BUILDER_CAPACITY: usize = 512 * 1024;

pub struct WebrenderRenderService {
    pango_context: pango::Context, // so that we can reuse already uploaded images
                                   // this can be (periodically) cleaned up by simply going through all keys and
                                   // looking what has (not) been used in the last render (and can be evicted)
                                   // _uploaded_images: BTreeMap<String, ImageKey>
}

impl WebrenderRenderService {
    pub fn new() -> Self {
        let font_map = FontMap::new().expect("couldn't get fontmap");
        let pango_context = pango::Context::new();
        pango_context.set_font_map(&font_map);

        WebrenderRenderService { pango_context }
    }
}

impl RenderService for WebrenderRenderService {
    fn render(&mut self, surface: &SurfaceData, computed_layouts: Vec<ComputedLayout>) {
        debug!("render\n{:#?}", surface);

        let content_size = LayoutSize::new(computed_layouts[0].2, computed_layouts[0].3);
        let pipeline_id = PipelineId::dummy();

        let mut context = RenderContext {
            computed_layouts,
            pango_context: &self.pango_context,

            builder: DisplayListBuilder::with_capacity(
                pipeline_id,
                content_size.clone(),
                BUILDER_CAPACITY,
            ),
            border_radius: WRBorderRadius::zero(),
            layout: LayoutPrimitiveInfo::new(content_size.into()),
            space_and_clip: SpaceAndClipInfo::root_scroll(pipeline_id),
        };

        context.render_surface(surface);

        temp::send_frame(context.builder)
    }
}

struct RenderContext<'a> {
    computed_layouts: Vec<ComputedLayout>,
    pango_context: &'a pango::Context,

    builder: DisplayListBuilder,
    border_radius: WRBorderRadius,
    layout: LayoutPrimitiveInfo,
    space_and_clip: SpaceAndClipInfo,
}

impl<'a> RenderContext<'a> {
    // TODO: scroll
    fn render_surface(&mut self, surface: &SurfaceData) {
        let parent_layout = self.layout;

        let (x, y, width, height) = self.computed_layouts[surface.id() as usize];

        self.layout = LayoutPrimitiveInfo::new(LayoutRect::new(
            LayoutPoint::new(
                parent_layout.rect.origin.x + x,
                parent_layout.rect.origin.y + y,
            ),
            LayoutSize::new(width, height),
        ));

        debug!("surface {} {:?}", surface.id(), self.layout.rect);

        // shared, not directly rendered
        // TODO: define & use clip (round or fixed?)
        if let Some(border_radius) = surface.border_radius() {
            self.border_radius = border_radius.clone().into();
        } else {
            self.border_radius = WRBorderRadius::zero();
        }

        // TODO: hittest

        if let Some(box_shadow) = surface.box_shadow() {
            let Vector2f(x, y) = box_shadow.offset;
            let size = box_shadow.spread + box_shadow.blur;
            let layout = LayoutPrimitiveInfo::with_clip_rect(
                self.layout.rect,
                self.layout
                    .rect
                    .translate(&TypedVector2D::new(x, y))
                    .inflate(size, size),
            );
            self.builder.push_item(
                &self.box_shadow(box_shadow.clone()),
                &layout,
                &self.space_and_clip,
            );
        }

        if let Some(color) = surface.background_color() {
            self.push(self.background_color(color.clone()));
        }

        if let Some(image) = surface.image() {
            self.push(self.image(image.clone()));
        }

        // TODO: selections (should be below text)

        if let Some(text) = surface.text() {
            let (item, glyphs) = self.text(text.clone());

            self.push(item);
            self.builder.push_iter(glyphs);
        }

        for child_surface in surface.children() {
            self.render_surface(&child_surface);
        }

        if let Some(border) = surface.border() {
            self.push(self.border(border.clone()));
        }

        // restore layout
        self.layout = parent_layout;
    }

    fn box_shadow(&self, box_shadow: BoxShadow) -> SpecificDisplayItem {
        SpecificDisplayItem::BoxShadow(BoxShadowDisplayItem {
            color: box_shadow.color.clone().into(),

            box_bounds: self.layout.rect,
            offset: box_shadow.offset.into(),
            blur_radius: box_shadow.blur,
            spread_radius: box_shadow.spread,
            border_radius: self.border_radius.clone().into(),

            // TODO: Inset/Outset (outset needs bigger clip-rect)
            clip_mode: BoxShadowClipMode::Outset,
        })
    }

    fn background_color(&self, color: Color) -> SpecificDisplayItem {
        SpecificDisplayItem::Rectangle(RectangleDisplayItem {
            color: color.into(),
        })
    }

    // TODO: refactor, cache, free + hook to make loading possible from node.js (http)
    fn image(&self, image: Image) -> SpecificDisplayItem {
        let image_key = temp::with_api(|api| {
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

            let key = api.generate_image_key();

            api.update_resources(vec![ResourceUpdate::AddImage(AddImage {
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
        });

        SpecificDisplayItem::Image(ImageDisplayItem {
            image_key,
            stretch_size: self.layout.rect.size.into(),
            tile_spacing: TypedSize2D::zero(),
            image_rendering: ImageRendering::Auto,
            alpha_type: AlphaType::PremultipliedAlpha,
            color: ColorF::WHITE,
        })
    }

    // TODO: cache, free, refactor, etc.
    // (this is rather PoC)
    fn text(&self, text: Text) -> (SpecificDisplayItem, Vec<GlyphInstance>) {
        let [text_x, text_y] = self.layout.rect.origin.to_array();

        let mut description = pango::FontDescription::new();
        description.set_family("Arial");
        description.set_size(text.font_size as i32);

        let layout = pango::Layout::new(self.pango_context);
        layout.set_font_description(&description);
        layout.set_wrap(WrapMode::Word);
        layout.set_width(100);
        layout.set_text(&text.text);

        let glyphs = temp::with_api(|render_api| {
            let mut glyphs: Vec<GlyphInstance> = Vec::new();

            let glyph_indices =
                render_api.get_glyph_indices(FontKey(IdNamespace(1), 1), &text.text);

            for (i, _char) in text.text.char_indices() {
                let rect = layout.index_to_pos(i as i32);

                if let Some(glyph_index) = glyph_indices[i] {
                    glyphs.push(GlyphInstance {
                        index: glyph_index,
                        point: LayoutPoint::new(
                            text_x + rect.x as f32,
                            30. + text_y + rect.y as f32,
                        ),
                    })
                }
            }

            glyphs
        });

        debug!("{:?}", &glyphs);

        let font_key = FontInstanceKey::new(IdNamespace(1), text.font_size as u32);

        let item = SpecificDisplayItem::Text(TextDisplayItem {
            font_key,
            color: text.color.clone().into(),
            glyph_options: None,
        });

        (item, glyphs)
    }

    fn border(&self, border: Border) -> SpecificDisplayItem {
        SpecificDisplayItem::Border(BorderDisplayItem {
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

    fn push(&mut self, item: SpecificDisplayItem) {
        debug!("push {:?}", &item);

        self.builder
            .push_item(&item, &self.layout, &self.space_and_clip);
    }
}

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
            bottom_right: LayoutSize::new(self.2, self.2),
            bottom_left: LayoutSize::new(self.3, self.3),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::Vector2f;

    fn test_ctx() -> SurfaceContext {
        // some "rect", optionally rounded (param to this fn?)

        SurfaceContext {
            border_radius: BorderRadius(0., 0., 0., 0.),
            layout: LayoutPrimitiveInfo::new(LayoutSize::new(100., 100.).into()),
        }
    }

    #[test]
    fn test_background_color() {
        let ctx = test_ctx();
        let color = Color(0, 0, 0, 255);

        assert_eq!(
            ctx.background_color(color.clone()),
            SpecificDisplayItem::Rectangle(RectangleDisplayItem {
                color: color.into()
            })
        );
    }

    #[test]
    fn test_box_shadow() {
        let ctx = test_ctx();
        let box_bounds = LayoutSize::new(100., 100.).into();
        let border_radius = BorderRadius(5., 5., 5., 5.);
        let color = Color(0, 0, 0, 255);
        let blur = 10.;
        let spread = 5.;
        let offset = Vector2f(5., 5.);
        let box_shadow = BoxShadow {
            offset: offset.clone(),
            blur,
            spread,
            color: color.clone(),
        };

        assert_eq!(
            ctx.box_shadow(box_shadow),
            SpecificDisplayItem::BoxShadow(BoxShadowDisplayItem {
                box_bounds,
                offset: offset.into(),
                color: color.into(),
                blur_radius: blur,
                spread_radius: spread,
                border_radius: border_radius.into(),
                clip_mode: BoxShadowClipMode::Outset
            })
        );
    }
}
