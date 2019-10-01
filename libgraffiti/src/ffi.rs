// bridge

use crate::text_layout::Text;
use crate::box_layout::{Layout, Overflow};
use crate::commons::{SurfaceId, Color, BorderRadius, Border, BoxShadow, Image};
use crate::app::TheApp;
use crate::window::{Event};
use miniserde::{json, Deserialize, Serialize};
use std::io::prelude::Write;

static mut APP: Option<TheApp> = None;

#[no_mangle]
pub extern "C" fn init() {
    unsafe { APP = Some(TheApp::init()) }
}

// returning the value would require javascript to copy it to the heap,
// we can avoid this simply by providing mutable ref to the already allocated
// (and possibly reused) memory
//
// - the result should be fixed size (no vecs), even when encoded in bincode
// - bincode encoding does not necessarily have to slow, it depends on the
//   shape of the result
// - often-occurring results should be "small" (Nothing, MouseMove)
#[no_mangle]
pub extern "C" fn send(data: *const u8, len: u32, mut result_ptr: &mut [u8]) {
    // get slice of bytes & try to deserialize
    let msg = unsafe { std::slice::from_raw_parts(data, len as usize) };
    let msg: FfiMsg = json::from_str(std::str::from_utf8(msg).expect("not string")).expect("invalid message");

    debug!("Msg {:#?}", &msg);

    /*
    // try to handle the message
    let maybe_err = std::panic::catch_unwind(|| unsafe {
        match APP {
            None => FfiResult::Nothing,
            Some(ref mut app) => handle_msg(app, msg),
        }
    });

    let result = maybe_err.unwrap_or_else(|err| {
        let err = err
            .downcast::<String>()
            .unwrap_or(Box::new("Unknown".into()));

        error!("err {:?}", err);

        FfiResult::Error(*err)
    });*/

    let result = FfiResult {
        events: Vec::new(),
        error: None,
    };

    result_ptr.write(json::to_string(&result).as_bytes()).expect("write result");
}

fn handle_msg(_app: &mut TheApp, _msg: FfiMsg) -> FfiResult {
    /*
    match msg {
        FfiMsg::GetEvents(poll) => FfiResult::Events(app.get_events(poll)),
        FfiMsg::CreateWindow => FfiResult::WindowId(app.create_window()),
        FfiMsg::UpdateScene { window, msgs } => {
            app.update_window_scene(window, &msgs);
            FfiResult::Nothing
        }

    }
    */
    FfiResult {
        events: Vec::new(),
        error: None,
    }
}

// some ffi-specific glue


#[derive(Deserialize, Serialize, Debug)]
pub struct FfiMsg {

}

#[derive(Deserialize, Serialize, Debug)]
pub struct FfiResult {
    events: Vec<Event>,
    error: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetLayout {
    surface: SurfaceId,
    layout: Option<Layout>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetBorderRadius {
    surface: SurfaceId,
    layout: Option<BorderRadius>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetBackgroundColor {
    surface: SurfaceId,
    color: Option<Color>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetBorder {
    surface: SurfaceId,
    border: Option<Border>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetBoxShadow {
    surface: SurfaceId,
    shadow: Option<BoxShadow>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetText {
    surface: SurfaceId,
    text: Option<Text>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetImage {
    surface: SurfaceId,
    image: Option<Image>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetOverflow {
    surface: SurfaceId,
    overflow: Overflow,
}

