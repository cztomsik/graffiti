use crate::api::{Window};
use crate::generated::Msg;
use crate::temp;
use crate::window::WindowImpl;
use serde_json;

static mut APP: Option<Box<App>> = None;

#[no_mangle]
pub extern "C" fn init() {
    temp::init();

    unsafe {
        APP = Some(Box::new(App {
            window: WindowImpl::new(),
        }))
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
                let window = &mut app.window;
                match msg {
                    Msg::HandleEvents => temp::handle_events(),

                    // all what this does is that it delegates to window.get_scene().* calls
                    // no logic should be here!
                    // TODO: maybe we could use some rust rpc which works with bincode (like https://github.com/google/tarpc)
                    _ => {
                        let scene = window.get_scene();

                        match msg {
                            Msg::Alloc => { scene.create_surface(); },
                            Msg::AppendChild { parent, child } => scene.append_child(parent, child),
                            Msg::InsertBefore {
                                parent,
                                child,
                                before,
                            } => {
                                scene.insert_before(parent, child, before);
                            }
                            Msg::RemoveChild { parent, child } => scene.remove_child(parent, child),
                            Msg::SetBorderRadius {
                                surface,
                                border_radius,
                            } => scene.set_border_radius(surface, border_radius),
                            Msg::SetSize { surface, size } => scene.set_size(surface, size),
                            Msg::SetFlow { surface, flow } => scene.set_flow(surface, flow),
                            Msg::SetFlex { surface, flex } => scene.set_flex(surface, flex),
                            Msg::SetPadding { surface, padding } => {
                                scene.set_padding(surface, padding)
                            }
                            Msg::SetMargin { surface, margin } => scene.set_margin(surface, margin),
                            Msg::SetBoxShadow {
                                surface,
                                box_shadow,
                            } => scene.set_box_shadow(surface, box_shadow),
                            Msg::SetBackgroundColor { surface, color } => {
                                scene.set_background_color(surface, color)
                            }
                            Msg::SetImage { surface, image } => scene.set_image(surface, image),
                            Msg::SetText { surface, text } => scene.set_text(surface, text),
                            Msg::SetBorder { surface, border } => scene.set_border(surface, border),
                            Msg::Render => window.render(),
                            _ => unreachable!()
                        };

                        window.render();
                    }
                }
            }
        }
    }
}

struct App {
    window: WindowImpl,
}
