mod generated;
mod storage;
mod surface;
mod render;
mod layout;
mod temp;

pub type Id = SurfaceId;

#[macro_use]
extern crate log;

use crate::generated::{Msg};
use serde_json;
use crate::layout::LayoutService;
use crate::surface::SurfaceService;
use crate::render::RenderService;
use crate::generated::SurfaceId;
use crate::layout::YogaLayoutService;
use crate::render::WebrenderRenderService;

static mut APP: Option<Box<App>> = None;

#[no_mangle]
pub extern "C" fn init() {
    temp::init();

    unsafe {
        APP = Some(Box::new(App {
            surface_service: SurfaceService::new(),
            layout_service: YogaLayoutService::new(),
            render_service: WebrenderRenderService::new(),
        }))
    }
}

#[no_mangle]
pub extern "C" fn send(data: *const u8, len: u32) {
    // get slice of bytes & try to deserialize
    let msg = unsafe { std::slice::from_raw_parts(data, len as usize) };
    let msg: Msg = serde_json::from_slice(msg).expect("invalid message");

    debug!("Msg {:#?}", &msg);

    unsafe {
        match APP {
            None => {}
            Some(ref mut app) => match msg {
                Msg::HandleEvents => temp::handle_events(),
                Msg::Alloc => {
                    app.surface_service.alloc();
                    app.layout_service.alloc();
                },
                Msg::AppendChild { parent, child } => {
                    app.surface_service.append_child(parent, child);
                    app.layout_service.append_child(parent, child);
                }
                Msg::InsertBefore { parent, child, before } => {
                    let data = app.surface_service.get_surface_data(parent);
                    let index = data.children().position(|child| child.id() == before).expect("not found");
                    app.surface_service.insert_before(parent, child, before);
                    app.layout_service.insert_at(parent, child, index as u32);
                }
                Msg::RemoveChild { parent, child } => {
                    app.surface_service.remove_child(parent, child);
                    app.layout_service.remove_child(parent, child);
                }
                Msg::SetBoxShadow {
                    surface,
                    box_shadow,
                } => app.surface_service.set_box_shadow(surface, box_shadow),
                Msg::SetBackgroundColor { surface, color } => {
                    app.surface_service.set_background_color(surface, color)
                }
                Msg::SetImage { surface, image } => {
                    app.surface_service.set_image(surface, image)
                }
                Msg::SetText { surface, text } => {
                    app.surface_service.set_text(surface, text)
                }
                Msg::SetBorder { surface, border } => {
                    app.surface_service.set_border(surface, border)
                }
                Msg::Render { surface } => {
                    let surface = app.surface_service.get_surface_data(surface);

                    app.render_service.render(&surface);
                }
                _ => {}
            }
        }
    }
}

struct App {
    surface_service: SurfaceService,
    layout_service: YogaLayoutService,
    render_service: WebrenderRenderService
}
