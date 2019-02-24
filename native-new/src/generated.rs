
use bincode;
use serde::Deserialize;


#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "tag", content = "value")]
pub enum Msg {
    CreateSurface,
    SurfaceMsg { surface: SurfaceId, msg: SurfaceMsg },
}


#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "tag", content = "value")]
pub enum SurfaceMsg {
    AppendChild { parent: SurfaceId, child: SurfaceId },
    InsertBefore { parent: SurfaceId, child: SurfaceId, before: SurfaceId },
    RemoveChild { parent: SurfaceId, child: SurfaceId },
    SetSize(Size),
    SetFlex(Flex),
    SetPadding(Rect),
    SetMargin(Rect),
    SetBoxShadow(Option<BoxShadow>),
    SetBackgroundColor(Option<BackgroundColor>),
    SetImage(Option<Image>),
    SetText(Option<Text>),
    SetBorder(Option<Border>),
}


#[derive(Deserialize, Debug, Clone)]
pub struct SurfaceId(pub u32);


#[derive(Deserialize, Debug, Clone)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);


#[derive(Deserialize, Debug, Clone)]
pub struct Flex {
    pub grow: f32,
    pub shrink: f32,
    pub basis: Dimension,
}


#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "tag", content = "value")]
pub enum Dimension {
    Auto,
    Point(f32),
    Percent(f32),
}


#[derive(Deserialize, Debug, Clone)]
pub struct Size(pub Dimension, pub Dimension);


#[derive(Deserialize, Debug, Clone)]
pub struct Rect(pub Dimension, pub Dimension, pub Dimension, pub Dimension);


#[derive(Deserialize, Debug, Clone)]
pub struct Vector2f(pub f32, pub f32);


#[derive(Deserialize, Debug, Clone)]
pub struct BoxShadow {
    pub color: Color,
    pub offset: Vector2f,
    pub blur: f32,
    pub spread: f32,
}


#[derive(Deserialize, Debug, Clone)]
pub struct BackgroundColor(pub Color);


#[derive(Deserialize, Debug, Clone)]
pub struct Image {
    pub url: String,
}


#[derive(Deserialize, Debug, Clone)]
pub struct Text {
    pub text: String,
}


#[derive(Deserialize, Debug, Clone)]
pub struct Border {
    pub top: BorderSide,
    pub right: BorderSide,
    pub bottom: BorderSide,
    pub left: BorderSide,
}


#[derive(Deserialize, Debug, Clone)]
pub struct BorderSide {
    pub color: Color,
    pub style: BorderStyle,
}


#[derive(Deserialize, Debug, Clone)]
pub enum BorderStyle {
    None,
    Solid,
}

