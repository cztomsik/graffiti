mod generated;

use bincode;
use serde::Deserialize;
use serde_json;
use yoga::{FlexStyle, Node as YogaNode, StyleUnit};

use crate::generated::*;

#[no_mangle]
pub extern "C" fn send(data: *const u8, len: u32) {
    // get slice of bytes & try to deserialize
    let msg = unsafe { std::slice::from_raw_parts(data, len as usize) };
    //let msg: Msg = bincode::deserialize(msg).expect("invalid message");
    let msg: Msg = serde_json::from_slice(msg).expect("invalid message");

    let mut surfaces: Vec<SurfaceData> = vec![SurfaceData {
        yoga_node: YogaNode::new(),
        box_shadow: None,
        background_color: None,
        image: None,
        text: None,
        border: None,
    }];

    match msg {
        Msg::CreateSurface => {}
        Msg::SurfaceMsg { surface, msg } => {
            let surface: &mut SurfaceData = surfaces
                .get_mut(surface.0 as usize)
                .expect("missing surface");

            match msg {
                SurfaceMsg::AppendChild { parent, child } => {}
                SurfaceMsg::InsertBefore {
                    parent,
                    child,
                    before,
                } => {}
                SurfaceMsg::RemoveChild { parent, child } => {}
                SurfaceMsg::SetSize(size) => surface.set_size(size),
                SurfaceMsg::SetFlex(flex) => surface.set_flex(flex),
                SurfaceMsg::SetPadding(padding) => surface.set_padding(padding),
                SurfaceMsg::SetMargin(margin) => surface.set_margin(margin),
                SurfaceMsg::SetBoxShadow(box_shadow) => surface.set_box_shadow(box_shadow),
                SurfaceMsg::SetBackgroundColor(color) => surface.set_background_color(color),
                SurfaceMsg::SetBorder(border) => surface.set_border(border),
                SurfaceMsg::SetText(text) => surface.set_text(text),
                SurfaceMsg::SetImage(image) => surface.set_image(image),
            }
        }
    }

    render(&surfaces[0]);
}

// TODO: rename
// TODO: children
#[derive(Debug)]
struct SurfaceData {
    // shape, so it does not have to repeated in shadow & border
    //border_radius: f32,

    // layout
    yoga_node: YogaNode,

    // optional components (in the sense of layers) of this surface
    box_shadow: Option<BoxShadow>,
    background_color: Option<Color>,
    image: Option<Image>,
    text: Option<Text>,
    border: Option<Border>,
}

impl SurfaceData {
    fn set_size(&mut self, size: Size) {
        self.yoga_node.apply_styles(&vec![
            FlexStyle::Width(size.0.into()),
            FlexStyle::Height(size.1.into()),
        ]);
    }

    fn set_flex(&mut self, flex: Flex) {
        self.yoga_node.apply_styles(&vec![
            FlexStyle::FlexGrow(flex.grow.into()),
            FlexStyle::FlexShrink(flex.shrink.into()),
            FlexStyle::FlexBasis(flex.basis.into()),
        ]);
    }

    fn set_padding(&mut self, padding: Rect) {
        self.yoga_node.apply_styles(&vec![
            FlexStyle::PaddingTop(padding.0.into()),
            FlexStyle::PaddingRight(padding.1.into()),
            FlexStyle::PaddingBottom(padding.2.into()),
            FlexStyle::PaddingLeft(padding.3.into()),
        ]);
    }

    fn set_margin(&mut self, margin: Rect) {
        self.yoga_node.apply_styles(&vec![
            FlexStyle::MarginTop(margin.0.into()),
            FlexStyle::MarginRight(margin.1.into()),
            FlexStyle::MarginBottom(margin.2.into()),
            FlexStyle::MarginLeft(margin.3.into()),
        ]);
    }

    fn set_box_shadow(&mut self, box_shadow: Option<BoxShadow>) {
        self.box_shadow = box_shadow;
    }

    fn set_background_color(&mut self, background_color: Option<Color>) {
        self.background_color = background_color;
    }

    fn set_image(&mut self, image: Option<Image>) {
        self.image = image;
    }

    fn set_text(&mut self, text: Option<Text>) {
        self.text = text;
    }

    fn set_border(&mut self, border: Option<Border>) {
        self.border = border;
    }
}

impl Into<StyleUnit> for Dimension {
    fn into(self) -> StyleUnit {
        match self {
            Dimension::Auto => StyleUnit::Auto,
            Dimension::Percent(f) => StyleUnit::Percent(f.into()),
            Dimension::Point(f) => StyleUnit::Point(f.into()),
        }
    }
}

fn render(surface: &SurfaceData) {
    println!("{:#?}", surface);
}
