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
use crate::surface::SurfaceService;
use crate::render::RenderService;
use crate::generated::SurfaceId;
use crate::layout::{LayoutService, YogaLayoutService};
use crate::render::WebrenderRenderService;
use crate::generated::Size;
use crate::generated::Dimension;

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

    debug!("Msg {:?}", &msg);

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
                Msg::SetBorderRadius{ surface, border_radius } => {
                    app.surface_service.set_border_radius(surface, border_radius );
                }
                Msg::SetSize { surface, size } => {
                    app.layout_service.set_size(surface, size);
                }
                Msg::SetFlow { surface, flow } => {
                    app.layout_service.set_flow(surface, flow );
                }
                Msg::SetFlex { surface, flex } => {
                    app.layout_service.set_flex(surface, flex );
                }
                Msg::SetPadding { surface, padding } => {
                    app.layout_service.set_padding(surface, padding );
                }
                Msg::SetMargin { surface, margin } => {
                    app.layout_service.set_margin(surface, margin );
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

                    // TODO: set on resize
                    let layout_size = temp::get_layout_size();
                    app.layout_service.set_size(0, Size(Dimension::Point(layout_size.width), Dimension::Point(layout_size.height)));

                    let computed_layouts = app.layout_service.get_computed_layouts(&surface);

                    app.render_service.render(&surface, computed_layouts);
                }
            }
        }
    }
}

struct App {
    surface_service: SurfaceService,
    layout_service: YogaLayoutService,
    render_service: WebrenderRenderService
}
