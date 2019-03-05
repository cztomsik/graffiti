use super::{
    Border, BorderRadius, BorderSide, BorderStyle, BoxShadow, Color, ComputedLayout, Image,
    RenderService, Text,
};
use crate::generated::Vector2f;
use crate::surface::SurfaceData;
use crate::temp::send_frame;
use webrender::api::{
    AlphaType, BorderDetails, BorderDisplayItem, BorderRadius as WRBorderRadius,
    BorderSide as WRBorderSide, BorderStyle as WRBorderStyle, BoxShadowClipMode,
    BoxShadowDisplayItem, ColorF, ColorU, DisplayListBuilder, FontInstanceKey, GlyphInstance,
    IdNamespace, ImageDisplayItem, ImageKey, ImageRendering, LayoutPoint, LayoutPrimitiveInfo,
    LayoutRect, LayoutSize, LayoutVector2D, NormalBorder, PipelineId, RectangleDisplayItem,
    SpaceAndClipInfo, SpecificDisplayItem, TextDisplayItem,
};
use webrender::euclid::{TypedSideOffsets2D, TypedSize2D};

static BUILDER_CAPACITY: usize = 512 * 1024;

pub struct WebrenderRenderService {}

impl WebrenderRenderService {
    pub fn new() -> Self {
        WebrenderRenderService {}
    }
}

impl RenderService for WebrenderRenderService {
    fn render(&mut self, surface: &SurfaceData, computed_layouts: Vec<ComputedLayout>) {
        debug!("render\n{:#?}", surface);

        let content_size = LayoutSize::new(computed_layouts[0].2, computed_layouts[0].3);
        let pipeline_id = PipelineId::dummy();

        let mut context = RenderContext {
            computed_layouts,

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

        send_frame(context.builder)
    }
}

struct RenderContext {
    computed_layouts: Vec<ComputedLayout>,

    builder: DisplayListBuilder,
    border_radius: WRBorderRadius,
    layout: LayoutPrimitiveInfo,
    space_and_clip: SpaceAndClipInfo,
}

impl RenderContext {
    fn render_surface(&mut self, surface: &SurfaceData) {
        let parent_layout = self.layout;

        let (x, y, width, height) = self.computed_layouts[surface.id() as usize];

        self.layout = LayoutPrimitiveInfo::new(LayoutRect::new(
            LayoutPoint::new(parent_layout.rect.origin.x + x, parent_layout.rect.origin.y + y),
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
            self.push(self.box_shadow(box_shadow.clone()));
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

            // TODO
            clip_mode: BoxShadowClipMode::Outset,
        })
    }

    fn background_color(&self, color: Color) -> SpecificDisplayItem {
        SpecificDisplayItem::Rectangle(RectangleDisplayItem {
            color: color.into(),
        })
    }

    fn image(&self, _image: Image) -> SpecificDisplayItem {
        // TODO
        let image_key = ImageKey::DUMMY;

        SpecificDisplayItem::Image(ImageDisplayItem {
            image_key,
            stretch_size: self.layout.rect.size.into(),
            tile_spacing: TypedSize2D::zero(),
            image_rendering: ImageRendering::Auto,
            alpha_type: AlphaType::PremultipliedAlpha,
            color: ColorF::WHITE,
        })
    }

    fn text(&self, text: Text) -> (SpecificDisplayItem, Vec<GlyphInstance>) {
        // TODO
        let font_key = FontInstanceKey::new(IdNamespace(0), 0);

        let item = SpecificDisplayItem::Text(TextDisplayItem {
            font_key,
            color: text.color.clone().into(),
            glyph_options: None,
        });

        // TODO
        let glyphs = vec![GlyphInstance {
            index: 40,
            point: self.layout.rect.origin.clone(),
        }];

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
