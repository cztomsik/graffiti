use crate::api::{Window, App};
use crate::generated::Msg;
use serde_json;
use crate::app::GlutinApp;

static mut APP: Option<GlutinApp> = None;

#[no_mangle]
pub extern "C" fn init() {
    env_logger::init();

    unsafe {
        let mut app = GlutinApp::new();

        app.create_window();

        APP = Some(app)
    }
}

#[no_mangle]
pub extern "C" fn send(data: *const u8, len: u32) {
    // get slice of bytes & try to deserialize
    let msg = unsafe { std::slice::from_raw_parts(data, len as usize) };
    let msg: Msg = serde_json::from_slice(msg).expect("invalid message");

    debug!("Msg {:?}", &msg);

    unsafe {
        match APP {
            None => {}
            Some(ref mut app) => {
                let window = app.get_window(app.get_first_window_id());
                match msg {
                    Msg::HandleEvents => {
                        // TODO: send next event
                    },

                    // all what this does is that it delegates to window.get_scene().* calls
                    // no logic should be here!
                    // TODO: maybe we could use some rust rpc which works with bincode (like https://github.com/google/tarpc)
                    // TODO: currently it does exact opposite of what we want (batch changes, one render)
                    msg => {
                        window.update_scene(|ctx| {
                            match msg.clone() {
                                Msg::Alloc => { ctx.create_surface(); },
                                Msg::AppendChild { parent, child } => ctx.append_child(parent, child),
                                Msg::InsertBefore {
                                    parent,
                                    child,
                                    before,
                                } => {
                                    ctx.insert_before(parent, child, before);
                                }
                                Msg::RemoveChild { parent, child } => ctx.remove_child(parent, child),
                                Msg::SetBorderRadius {
                                    surface,
                                    border_radius,
                                } => ctx.set_border_radius(surface, border_radius),
                                Msg::SetSize { surface, size } => ctx.set_size(surface, size),
                                Msg::SetFlow { surface, flow } => ctx.set_flow(surface, flow),
                                Msg::SetFlex { surface, flex } => ctx.set_flex(surface, flex),
                                Msg::SetPadding { surface, padding } => {
                                    ctx.set_padding(surface, padding)
                                }
                                Msg::SetMargin { surface, margin } => ctx.set_margin(surface, margin),
                                Msg::SetBoxShadow {
                                    surface,
                                    box_shadow,
                                } => ctx.set_box_shadow(surface, box_shadow),
                                Msg::SetBackgroundColor { surface, color } => {
                                    ctx.set_background_color(surface, color)
                                }
                                Msg::SetImage { surface, image } => ctx.set_image(surface, image),
                                Msg::SetText { surface, text } => ctx.set_text(surface, text),
                                Msg::SetBorder { surface, border } => ctx.set_border(surface, border),
                                Msg::Render => panic!("render is now called automatially"),

                                // already covered
                                Msg::HandleEvents => unreachable!()
                            };
                        })
                    }
                }
            }
        }
    }
}
