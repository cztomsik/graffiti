// bridge

use crate::app::{TheApp, WindowId};
use crate::window::{Event, UpdateSceneMsg};
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

    silly!("Msg {:#?}", &msg);

    // try to handle the message
    let maybe_err = std::panic::catch_unwind(|| unsafe {
        match APP {
            None => panic!("no app"),
            Some(ref mut app) => handle_msg(app, &msg),
        }
    });

    let result = maybe_err.unwrap_or_else(|err| {
        let err = err
            .downcast::<String>()
            .unwrap_or(Box::new("Unknown".into()))
            .to_string();

        error!("err {:?}", err);

        FfiResult {
            events: Vec::new(),
            error: Some(err)
        }
    });

    result_ptr.write(json::to_string(&result).as_bytes()).expect("write result");
}

fn handle_msg(app: &mut TheApp, msg: &FfiMsg) -> FfiResult {
    // TODO: think more about windows, support closing

    let window_id = msg.window.unwrap_or_else(|| app.create_window());
    let events;

    // TODO: maybe we can both update and get events
    // but it would need some changes in js
    if let Some(update_msg) = &msg.update {
        app.update_window_scene(window_id, update_msg);
        events = Vec::new();
    } else {
        events = app.get_events(false);
    }

    FfiResult {
        events,
        error: None,
    }
}

// some ffi-specific glue

#[derive(Deserialize, Serialize, Debug)]
pub struct FfiMsg {
    window: Option<WindowId>,
    update: Option<UpdateSceneMsg>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FfiResult {
    // TODO: multi-window
    events: Vec<Event>,
    error: Option<String>
}
