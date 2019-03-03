
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
    SetBorderRadius { surface: SurfaceId, border_radius: Option<BorderRadius> },
    SetSize { surface: SurfaceId, size: Size },
    SetFlex { surface: SurfaceId, flex: Flex },
    SetFlow { surface: SurfaceId, flow: Flow },
    SetPadding { surface: SurfaceId, padding: Rect },
    SetMargin { surface: SurfaceId, margin: Rect },
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
pub enum FlexDirection {
    Column,
    ColumnReverse,
    Row,
    RowReverse,
}


#[derive(Deserialize, Debug, Clone)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}


#[derive(Deserialize, Debug, Clone)]
pub enum FlexAlign {
    Auto,
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
    Baseline,
    SpaceBetween,
    SpaceAround,
}


#[derive(Deserialize, Debug, Clone)]
pub enum JustifyContent {
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}


#[derive(Deserialize, Debug, Clone)]
pub struct Flow {
    #[serde(rename = "flexDirection")]
    pub flex_direction: FlexDirection,

    #[serde(rename = "flexWrap")]
    pub flex_wrap: FlexWrap,

    #[serde(rename = "alignContent")]
    pub align_content: FlexAlign,

    #[serde(rename = "alignItems")]
    pub align_items: FlexAlign,

    #[serde(rename = "justifyContent")]
    pub justify_content: JustifyContent,

}


#[derive(Deserialize, Debug, Clone)]
pub struct Flex {
    #[serde(rename = "flexGrow")]
    pub flex_grow: f32,

    #[serde(rename = "flexShrink")]
    pub flex_shrink: f32,

    #[serde(rename = "flexBasis")]
    pub flex_basis: Dimension,

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
pub struct BorderRadius(pub f32, pub f32, pub f32, pub f32);


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
    #[serde(rename = "fontSize")]
    pub font_size: f32,

    #[serde(rename = "lineHeight")]
    pub line_height: f32,

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

