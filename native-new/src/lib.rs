mod generated;

use bincode;
use serde_json;
use serde::Deserialize;

use crate::generated::*;

#[no_mangle]
pub extern "C" fn send(data: *const u8, len: u32) {
    // get slice of bytes & try to deserialize
    let msg = unsafe { std::slice::from_raw_parts(data, len as usize) };
    //let msg: Msg = bincode::deserialize(msg).expect("invalid message");
    let msg: Msg = serde_json::from_slice(msg).expect("invalid message");

    println!("Got {:?}", msg);
}


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
