
use bincode;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
#[serde(tag = "tag", content = "value")]
pub enum Msg {
    Hello,
    World(String),
    All { people: String },
}


#[derive(Deserialize, Debug)]
#[serde(tag = "tag", content = "value")]
pub enum X {
    AppendChild { parent: SurfaceId, child: SurfaceId },
    RemoveChild { parent: SurfaceId, child: SurfaceId },
}


#[derive(Deserialize, Debug)]
pub struct SurfaceId(u32);


#[derive(Deserialize, Debug)]
pub struct SurfaceCanHave {
    #[serde(rename = "borderRadius")]
    border_radius: f32,

    #[serde(rename = "boxShadow")]
    box_shadow: Option<BoxShadow>,

    #[serde(rename = "backgroundColor")]
    background_color: Option<Color>,

    #[serde(rename = "backgroundImage")]
    background_image: Option<Image>,

    text: Option<Text>,
    border: Option<Border>,
}


#[derive(Deserialize, Debug)]
pub struct Color(f32, f32, f32, f32);


#[derive(Deserialize, Debug)]
pub struct Vector2f(f32, f32);


#[derive(Deserialize, Debug)]
pub struct BoxShadow {
    color: Color,
    offset: Vector2f,
    blur: f32,
    spread: f32,
}


#[derive(Deserialize, Debug)]
pub struct Image {
    url: String,
}


#[derive(Deserialize, Debug)]
pub struct Text {
    text: String,
}


#[derive(Deserialize, Debug)]
pub struct Border {
    top: BorderSide,
    right: BorderSide,
    bottom: BorderSide,
    left: BorderSide,
}


#[derive(Deserialize, Debug)]
pub struct BorderSide {
    width: f32,
    color: Color,
    style: BorderStyle,
}


#[derive(Deserialize, Debug)]
pub enum BorderStyle {
    None,
    Solid,
}

