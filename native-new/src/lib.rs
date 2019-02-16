use bincode;
use serde::Deserialize;

use tscodegen::tscodegen;

#[no_mangle]
pub extern "C" fn send(data: *const u8, len: u32) {
    // get slice of bytes & try to deserialize
    let msg = unsafe { std::slice::from_raw_parts(data, len as usize) };
    let msg: Msg = bincode::deserialize(msg).expect("invalid message");

    println!("Got {:?}", msg);
}

#[tscodegen]
#[derive(Deserialize, Debug)]
pub enum Msg {
    Hello,
    World,
}

//#[tscodegen]
#[derive(Deserialize, Debug)]
pub enum X {
    AppendChild { parent: SurfaceId, child: SurfaceId },
    RemoveChild { parent: SurfaceId, child: SurfaceId },
}

#[tscodegen]
#[derive(Deserialize, Debug)]
pub struct SurfaceId(u32);





struct SurfaceCanHave {
    // shape, so it does not have to repeated in shadow & border
    border_radius: f32,

    // TODO: layout
    // TODO: children

    // & some optional parts
    box_shadow: Option<BoxShadow>,
    background_color: Option<Color>,
    background_image: Option<Image>,
    text: Option<Text>,
    border: Option<Border>
}

#[tscodegen]
#[derive(Deserialize, Debug)]
pub struct BoxShadow {
    color: Color,
    offset: (f32, f32),
    blur: f32,
    spread: f32
}

#[tscodegen]
#[derive(Deserialize, Debug)]
pub struct Border {
    top: BorderSide,
    right: BorderSide,
    bottom: BorderSide,
    left: BorderSide
}

#[tscodegen]
#[derive(Deserialize, Debug)]
pub struct BorderSide {
    width: f32,
    color: Color,
    style: BorderStyle
}

#[tscodegen]
#[derive(Deserialize, Debug)]
pub enum BorderStyle {
    None,
    Solid
}

#[tscodegen]
#[derive(Deserialize, Debug)]
pub struct Color(f32, f32, f32, f32);

#[tscodegen]
#[derive(Deserialize, Debug)]
pub struct Image {
    url: String
}

#[tscodegen]
#[derive(Deserialize, Debug)]
pub struct Text {
    text: String
}
