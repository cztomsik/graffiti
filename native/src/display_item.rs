extern crate webrender;
extern crate serde;
extern crate serde_json;

use webrender::api::{DisplayListBuilder, LayoutPrimitiveInfo, ColorF, FontInstanceKey, GlyphInstance};

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum DisplayItem {
    Rect { info: LayoutPrimitiveInfo, color: ColorF },
    Text { info: LayoutPrimitiveInfo, font_instance_key: FontInstanceKey, glyphs: Vec<GlyphInstance>, color: ColorF }
}

impl DisplayItem {
    pub fn apply(&self, builder: &mut DisplayListBuilder) {
        let b = builder;

        println!("apply {}", serde_json::to_string_pretty(&self).unwrap());

        match *self {
            DisplayItem::Rect { info, color } => b.push_rect(&info, color),
            DisplayItem::Text { info, font_instance_key, ref glyphs, color } => b.push_text(&info, &glyphs, font_instance_key, color, None)
        }
    }

    pub fn parse_all_from_json(data: &str) -> Vec<DisplayItem> {
        serde_json::from_str(data).unwrap()
    }
}
