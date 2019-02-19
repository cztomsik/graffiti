
use bincode;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub enum Msg {
    Hello,
    World(String),
    You(u32, bool),
    All { people: String },
}


#[derive(Deserialize, Debug)]
pub enum X {
    AppendChild { parent: SurfaceId, child: SurfaceId },
    RemoveChild { parent: SurfaceId, child: SurfaceId },
}


#[derive(Deserialize, Debug)]
pub struct SurfaceId(u32);


#[derive(Deserialize, Debug)]
pub struct SurfaceCanHave {
    borderRadius: f32,
    boxShadow: Option<BoxShadow>,
    backgroundColor: Option<Color>,
    backgroundImage: Option<Image>,
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

