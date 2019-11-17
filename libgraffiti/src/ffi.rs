// bridge

use crate::app::{TheApp, WindowId};
use crate::window::{Event, UpdateSceneMsg};
use miniserde::{json, Deserialize, Serialize};
use std::io::prelude::Write;

// returning the value would require javascript to copy it to the heap,
// we can avoid this simply by providing mutable ref to the already allocated
// (and possibly reused) memory
//
// we dont need ffi anymore but it might be useful for future targets
#[no_mangle]
pub unsafe extern "C" fn gft_send(data: *const u8, len: usize, res_data: *mut u8, res_maxlen: usize) {
    // get slice of bytes & try to deserialize
    let msg = std::slice::from_raw_parts(data, len as usize);
    let msg = std::str::from_utf8(msg).expect("not string");
    let msg: FfiMsg = json::from_str(&msg).unwrap_or_else(|_| {
        panic!("invalid message {}", &msg);
    });

    silly!("Msg {:#?}", &msg);

    // try to handle the message
    let maybe_err = std::panic::catch_unwind(|| {
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

    let mut res_buf = std::slice::from_raw_parts_mut(res_data, res_maxlen);
    res_buf.write(json::to_string(&result).as_bytes()).expect("write result");
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
        events = app.get_events(msg.poll);
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
    poll: bool,
    update: Option<UpdateSceneMsg>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FfiResult {
    // TODO: multi-window
    events: Vec<Event>,
    error: Option<String>
}
