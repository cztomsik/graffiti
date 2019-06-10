use crate::app::TheApp;
use crate::generated::{FfiMsg, FfiResult, StyleProp, UpdateSceneMsg};
use bincode::{deserialize, serialize, serialize_into};
use serde_json;
use std::io::prelude::Write;

static mut APP: Option<TheApp> = None;

#[no_mangle]
pub extern "C" fn init() {
    env_logger::init();

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
pub extern "C" fn send(data: *const u8, len: u32, result_ptr: *mut u8) {
    // get slice of bytes & try to deserialize
    let msg = unsafe { std::slice::from_raw_parts(data, len as usize) };
    let msg: FfiMsg = deserialize(msg).expect("invalid message"); // serde_json::from_slice(msg).expect("invalid message");

    debug!("Msg {:#?}", &msg);

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
    });

    // serialize & write the result
    unsafe {
        // TODO: find a way to avoid memcpy
        // (it's not possible to use to_writer, because it takes ownership and so it would free the vec & the data)
        // let data = serde_json::to_vec(&result).expect("couldn't serialize result");
        let data = serialize(&result).expect("couldn't serialize result");
        let mut writer = Vec::from_raw_parts(result_ptr, 0, 1024);
        writer.write(&data[..]).unwrap();

        // serialize_into(writer, &result).unwrap();
        Box::leak(Box::new(writer));
    }
}

fn handle_msg(app: &mut TheApp, msg: FfiMsg) -> FfiResult {
    match msg {
        FfiMsg::GetEvents(poll) => FfiResult::Events(app.get_events(poll)),
        FfiMsg::CreateWindow => FfiResult::WindowId(app.create_window()),
        FfiMsg::UpdateScene { window, msgs } => {
            app.get_window_mut(window).update_scene(&msgs);
            FfiResult::Nothing
        }
    }
}
