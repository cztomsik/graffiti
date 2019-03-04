use super::{BorderRadius, Border, BoxShadow, Color, ComputedLayout, Image, RenderService, Text};
use webrender::api::{
    AlphaType, BorderDetails, BorderDisplayItem, BorderRadius as WRBorderRadius, BorderSide, BorderStyle,
    BoxShadowClipMode, BoxShadowDisplayItem, ColorF, ColorU, DisplayListBuilder, FontInstanceKey,
    IdNamespace, ImageDisplayItem, ImageKey, ImageRendering, LayoutPrimitiveInfo, LayoutRect,
    LayoutSize, NormalBorder, PipelineId, RectangleDisplayItem, SpaceAndClipInfo,
    SpecificDisplayItem, TextDisplayItem, LayoutPoint, GlyphInstance
};
use webrender::euclid::{TypedPoint2D, TypedSideOffsets2D, TypedSize2D};
use crate::surface::SurfaceData;
use crate::temp::send_frame;
use std::fmt::Debug;

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
            builder: DisplayListBuilder::with_capacity(pipeline_id, content_size.clone(), BUILDER_CAPACITY),
            border_radius: None,
            // TODO: clip (normal, border-radius, scrollframe)
            layout: LayoutPrimitiveInfo::new(content_size.into()),
            space_and_clip: SpaceAndClipInfo::root_scroll(pipeline_id)
        };

        context.render_surface(surface);

        send_frame(context.builder)
    }
}

struct RenderContext<'a> {
    computed_layouts: Vec<ComputedLayout>,
    builder: DisplayListBuilder,
    border_radius: Option<&'a BorderRadius>,
    layout: LayoutPrimitiveInfo,
    space_and_clip: SpaceAndClipInfo
}

impl <'a> RenderContext<'a> {
    fn render_surface(&mut self, surface: &SurfaceData) {
        let (x, y, width, height) = self.computed_layouts[surface.id() as usize];

        self.layout = LayoutPrimitiveInfo::new(LayoutRect::new(LayoutPoint::new(self.layout.rect.origin.x + x, self.layout.rect.origin.y + y), LayoutSize::new(width, height)));

        debug!("surface {} {:?}", surface.id(), &self.layout);

        // shared, not directly rendered
        // TODO: define & use clip
        // TODO: borrowing (radius should only be visible during rendering of one particular surface)
        self.border_radius = { surface.border_radius(); None };

        // TODO: hittest

        self.render_item(surface.box_shadow());

        if let Some(color) = surface.background_color() {
            self.render_item(Some(&BackgroundColor(color.clone())));
        }

        self.render_item(surface.image());

        // selection should be below text
        // TODO
        // render_item(data.selections.get(&id), &mut builder, &layout, &space_and_clip);

        self.render_item(surface.text());

        for child_surface in surface.children() {
            // TODO: layout, offset, space_and_clip, border_radius
            self.render_surface(&child_surface);
        }

        self.render_item(surface.border());
    }

    fn render_item<T>(&mut self,
        item: Option<&T>
    ) where T: RenderItem + Debug {
        if let Some(item) = item {
            debug!("render_item {:?}", &item);
            item.push_into(&mut self.builder, &self.layout, &self.space_and_clip);
        }
    }
}


trait RenderItem {
    // so that we can test it
    fn render(&self) -> SpecificDisplayItem;

    // so that we can customize it if necessary (push_iter() for text)
    // NOTE: trait can be resolved statically (in contrast to match clause)
    fn push_into(
        &self,
        builder: &mut DisplayListBuilder,
        layout: &LayoutPrimitiveInfo,
        space_and_clip: &SpaceAndClipInfo,
    ) {
        builder.push_item(&self.render(), layout, space_and_clip);
    }
}

impl RenderItem for BoxShadow {
    fn render(&self) -> SpecificDisplayItem {
        SpecificDisplayItem::BoxShadow(BoxShadowDisplayItem {
            color: self.color.clone().into(),

            // TODO
            box_bounds: LayoutRect::new(TypedPoint2D::new(0., 0.), TypedSize2D::new(100., 100.)),
            offset: [self.offset.0, self.offset.1].into(),
            blur_radius: self.blur,
            spread_radius: self.spread,

            // TODO
            border_radius: WRBorderRadius::uniform(5.0),

            // TODO
            clip_mode: BoxShadowClipMode::Outset,
        })
    }
}

// just so that we can implement the trait
#[derive(Debug)]
struct BackgroundColor(Color);

impl RenderItem for BackgroundColor {
    fn render(&self) -> SpecificDisplayItem {
        SpecificDisplayItem::Rectangle(RectangleDisplayItem {
            color: self.0.clone().into(),
        })
    }
}

// TODO
impl RenderItem for Text {
    fn render(&self) -> SpecificDisplayItem {
        // TODO
        let font_key = FontInstanceKey::new(IdNamespace(0), 0);

        SpecificDisplayItem::Text(TextDisplayItem {
            font_key,
            color: self.color.clone().into(),
            glyph_options: None,
        })
    }

    fn push_into(
        &self,
        builder: &mut DisplayListBuilder,
        layout: &LayoutPrimitiveInfo,
        space_and_clip: &SpaceAndClipInfo,
    ) {
        builder.push_item(&self.render(), layout, space_and_clip);

        // TODO
        let glyphs = vec![GlyphInstance { index: 40, point: layout.rect.origin.clone() }];

        builder.push_iter(&glyphs);
    }
}

impl RenderItem for Image {
    fn render(&self) -> SpecificDisplayItem {
        // TODO
        let image_key = ImageKey::DUMMY;
        let layout = LayoutPrimitiveInfo::new(TypedSize2D::new(0., 0.).into());

        SpecificDisplayItem::Image(ImageDisplayItem {
            image_key,
            stretch_size: layout.clone().rect.size,
            tile_spacing: TypedSize2D::zero(),
            image_rendering: ImageRendering::Auto,
            alpha_type: AlphaType::PremultipliedAlpha,
            color: ColorF::WHITE,
        })
    }
}

impl RenderItem for Border {
    // TODO: border-radius
    // TODO: border widths + colors + styles (actual border)
    fn render(&self) -> SpecificDisplayItem {
        // TODO: widths
        let top = 0.;
        let right = 0.;
        let bottom = 0.;
        let left = 0.;
        let widths = TypedSideOffsets2D::new(top, right, bottom, left);

        // TODO: colors + styles
        let details = BorderDetails::Normal(NormalBorder {
            top: BorderSide {
                color: ColorF::new(0., 0., 0., 0.),
                style: BorderStyle::Solid,
            },
            right: BorderSide {
                color: ColorF::new(0., 0., 0., 0.),
                style: BorderStyle::Solid,
            },
            bottom: BorderSide {
                color: ColorF::new(0., 0., 0., 0.),
                style: BorderStyle::Solid,
            },
            left: BorderSide {
                color: ColorF::new(0., 0., 0., 0.),
                style: BorderStyle::Solid,
            },
            // TODO
            radius: WRBorderRadius::uniform(5.0),
            do_aa: true,
        });

        SpecificDisplayItem::Border(BorderDisplayItem { widths, details })
    }
}

impl Into<ColorF> for Color {
    fn into(self) -> ColorF {
        let Color(r, g, b, a) = self;
        ColorU::new(r, g, b, a).into()
    }
}