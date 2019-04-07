use crate::api::App;
use crate::app::TheApp;
use crate::generated::{FfiMsg, FfiResult, UpdateSceneMsg};
use bincode::deserialize;
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

    unsafe {
        match APP {
            None => {}
            Some(ref mut app) => {
                let mut result = FfiResult::Nothing;

                handle_msg(app, msg, &mut result);

                // TODO: bincode
                // TODO: find a way to avoid memcpy
                // (it's not possible to use to_writer, because it takes ownership and so it would free the vec & the data)
                let data = serde_json::to_vec(&result).expect("couldn't serialize result");
                let mut writer = Vec::from_raw_parts(result_ptr, 0, 1024);
                writer.write(&data[..]).unwrap();
                Box::leak(Box::new(writer));
            }
        }
    }
}

fn handle_msg(app: &mut TheApp, msg: FfiMsg, result: &mut FfiResult) {
    match msg {
        FfiMsg::GetNextEvent(poll) => {
            if let Some(event) = app.get_next_event(poll) {
                *result = FfiResult::Event(event);
            }
        }
        FfiMsg::CreateWindow => {
            let id = app.create_window();
            *result = FfiResult::WindowId(id);
        }
        FfiMsg::UpdateScene { window, msgs } => {
            let window = app.get_window_mut(window);
            let ctx = window.scene_mut();

            // this should only delegate to appropriate ctx.* calls
            // no logic should be here!
            for msg in msgs {
                match msg {
                    UpdateSceneMsg::Alloc => {
                        ctx.create_surface();
                    }
                    UpdateSceneMsg::AppendChild { parent, child } => {
                        ctx.append_child(parent, child)
                    }
                    UpdateSceneMsg::InsertBefore {
                        parent,
                        child,
                        before,
                    } => {
                        ctx.insert_before(parent, child, before);
                    }
                    UpdateSceneMsg::RemoveChild { parent, child } => {
                        ctx.remove_child(parent, child)
                    }
                    UpdateSceneMsg::SetBorderRadius {
                        surface,
                        border_radius,
                    } => ctx.set_border_radius(surface, border_radius),
                    UpdateSceneMsg::SetSize { surface, size } => ctx.set_size(surface, size),
                    UpdateSceneMsg::SetFlow { surface, flow } => ctx.set_flow(surface, flow),
                    UpdateSceneMsg::SetFlex { surface, flex } => ctx.set_flex(surface, flex),
                    UpdateSceneMsg::SetPadding { surface, padding } => {
                        ctx.set_padding(surface, padding)
                    }
                    UpdateSceneMsg::SetMargin { surface, margin } => {
                        ctx.set_margin(surface, margin)
                    }
                    UpdateSceneMsg::SetBoxShadow {
                        surface,
                        box_shadow,
                    } => ctx.set_box_shadow(surface, box_shadow),
                    UpdateSceneMsg::SetBackgroundColor { surface, color } => {
                        ctx.set_background_color(surface, color)
                    }
                    UpdateSceneMsg::SetImage { surface, image } => ctx.set_image(surface, image),
                    UpdateSceneMsg::SetText { surface, text } => ctx.set_text(surface, text),
                    UpdateSceneMsg::SetBorder { surface, border } => {
                        ctx.set_border(surface, border)
                    }
                }
            }

            window.render();
        }
    }
}
