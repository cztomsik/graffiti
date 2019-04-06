
use bincode;
use serde::{Serialize, Deserialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "tag", content = "value")]
pub enum FfiMsg {
    GetNextEvent(bool),
    CreateWindow,
    UpdateScene { window: WindowId, msgs: Vec<UpdateSceneMsg> },
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "tag", content = "value")]
pub enum FfiResult {
    Nothing,
    Event(Event),
    WindowId(WindowId),
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "tag", content = "value")]
pub enum Event {
    WindowEvent { window: WindowId, event: WindowEvent },
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "tag", content = "value")]
pub enum WindowEvent {
    MouseMove { target: usize },
    MouseDown,
    MouseUp,
    KeyDown,
    KeyPress(u16),
    KeyUp,
    Focus,
    Blur,
    Resize,
    Close,
    Unknown,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "tag", content = "value")]
pub enum UpdateSceneMsg {
    Alloc,
    AppendChild { parent: SurfaceId, child: SurfaceId },
    InsertBefore { parent: SurfaceId, child: SurfaceId, before: SurfaceId },
    RemoveChild { parent: SurfaceId, child: SurfaceId },
    SetBorderRadius { surface: SurfaceId, #[serde(rename = "borderRadius")] border_radius: Option<BorderRadius> },
    SetSize { surface: SurfaceId, size: Size },
    SetFlex { surface: SurfaceId, flex: Flex },
    SetFlow { surface: SurfaceId, flow: Flow },
    SetPadding { surface: SurfaceId, padding: Dimensions },
    SetMargin { surface: SurfaceId, margin: Dimensions },
    SetBoxShadow { surface: SurfaceId, #[serde(rename = "boxShadow")] box_shadow: Option<BoxShadow> },
    SetBackgroundColor { surface: SurfaceId, color: Option<Color> },
    SetImage { surface: SurfaceId, image: Option<Image> },
    SetText { surface: SurfaceId, text: Option<Text> },
    SetBorder { surface: SurfaceId, border: Option<Border> },
}


pub type WindowId = u16;


pub type SurfaceId = usize;


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);


#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum FlexDirection {
    Column,
    ColumnReverse,
    Row,
    RowReverse,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
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


#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum JustifyContent {
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
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


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Flex {
    #[serde(rename = "flexGrow")]
    pub flex_grow: f32,

    #[serde(rename = "flexShrink")]
    pub flex_shrink: f32,

    #[serde(rename = "flexBasis")]
    pub flex_basis: Dimension,

}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "tag", content = "value")]
pub enum Dimension {
    Auto,
    Point(f32),
    Percent(f32),
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Size(pub Dimension, pub Dimension);


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Rect(pub f32, pub f32, pub f32, pub f32);


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Dimensions(pub Dimension, pub Dimension, pub Dimension, pub Dimension);


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Vector2f(pub f32, pub f32);


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BorderRadius(pub f32, pub f32, pub f32, pub f32);


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BoxShadow {
    pub color: Color,
    pub offset: Vector2f,
    pub blur: f32,
    pub spread: f32,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Image {
    pub url: String,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Text {
    pub color: Color,
    #[serde(rename = "fontSize")]
    pub font_size: f32,

    #[serde(rename = "lineHeight")]
    pub line_height: f32,

    pub align: TextAlign,
    pub text: String,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Border {
    pub top: BorderSide,
    pub right: BorderSide,
    pub bottom: BorderSide,
    pub left: BorderSide,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BorderSide {
    pub width: f32,
    pub style: BorderStyle,
    pub color: Color,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BorderStyle {
    None,
    Solid,
}

