use crate::api::{App, Window, WindowId};
use crate::app::GlutinApp;
use crate::generated::{FfiMsg, UpdateSceneMsg};
use serde_json;

static mut APP: Option<GlutinApp> = None;

#[no_mangle]
pub extern "C" fn init() {
    env_logger::init();

    unsafe { APP = Some(GlutinApp::new()) }
}

#[no_mangle]
pub extern "C" fn send(data: *const u8, len: u32, result_ptr: *mut FfiResult) {
    // get slice of bytes & try to deserialize
    let msg = unsafe { std::slice::from_raw_parts(data, len as usize) };
    let msg: FfiMsg = serde_json::from_slice(msg).expect("invalid message");

    debug!("Msg {:#?}", &msg);

    unsafe {
        match APP {
            None => {}
            Some(ref mut app) => {
                let mut result = FfiResult::Ok;

                handle_msg(app, msg, &mut result);

                *result_ptr = result;
            }
        }
    }
}

fn handle_msg(app: &mut GlutinApp, msg: FfiMsg, result: &mut FfiResult) {
    match msg {
        FfiMsg::HandleEvents => {
            // TODO: return AppEvent
            app.get_next_event();
        }
        FfiMsg::CreateWindow => {
            let id = app.create_window();
            *result = FfiResult::WindowId(id);
        }
        FfiMsg::UpdateScene { window, msgs } => {
            let window = app.get_window(window);

            // this should only delegate to appropriate ctx.* calls
            // no logic should be here!
            window.update_scene(|ctx| {
                for msg in msgs.clone() {
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
                        UpdateSceneMsg::SetImage { surface, image } => {
                            ctx.set_image(surface, image)
                        }
                        UpdateSceneMsg::SetText { surface, text } => ctx.set_text(surface, text),
                        UpdateSceneMsg::SetBorder { surface, border } => {
                            ctx.set_border(surface, border)
                        }
                    }
                }
            })
        }
    }
}

pub enum FfiResult {
    Ok,
    WindowId(WindowId)
}
