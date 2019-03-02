
use bincode;
use serde::Deserialize;


#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "tag", content = "value")]
pub enum Msg {
    HandleEvents,
    Alloc,
    AppendChild { parent: SurfaceId, child: SurfaceId },
    InsertBefore { parent: SurfaceId, child: SurfaceId, before: SurfaceId },
    RemoveChild { parent: SurfaceId, child: SurfaceId },
    SetSize { surface: SurfaceId, size: Size },
    SetFlex { surface: SurfaceId, flex: Flex },
    SetPadding { surface: SurfaceId, rect: Rect },
    SetMargin { surface: SurfaceId, rect: Rect },
    SetBoxShadow { surface: SurfaceId, box_shadow: Option<BoxShadow> },
    SetBackgroundColor { surface: SurfaceId, color: Option<Color> },
    SetImage { surface: SurfaceId, image: Option<Image> },
    SetText { surface: SurfaceId, text: Option<Text> },
    SetBorder { surface: SurfaceId, border: Option<Border> },
    Render { surface: SurfaceId },
}


pub type SurfaceId = u16;


#[derive(Deserialize, Debug, Clone)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);


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
pub struct Image {
    pub url: String,
}


#[derive(Deserialize, Debug, Clone)]
pub struct Text {
    pub color: Color,
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
    pub width: f32,
    pub style: BorderStyle,
    pub color: Color,
}


#[derive(Deserialize, Debug, Clone)]
pub enum BorderStyle {
    None,
    Solid,
}

