mod generated;
mod surface;
mod render;
mod layout;
mod temp;

// TODO: generics
// (into() is not enough; maybe struct)
// usize for now
pub type Id = usize;

use crate::generated::{Msg};
use serde::Deserialize;
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

    unsafe { APP = Some(Box::new(App {
        surface_service: SurfaceService::new(),
        layout_service: YogaLayoutService::new(),
        render_service: WebrenderRenderService::new()
    })) }
}

#[no_mangle]
pub extern "C" fn send(data: *const u8, len: u32) {
    // get slice of bytes & try to deserialize
    let msg = unsafe { std::slice::from_raw_parts(data, len as usize) };
    let msg: Msg = serde_json::from_slice(msg).expect("invalid message");

    unsafe {
        match APP {
            None => {}
            Some(ref mut app) => match msg {
                Msg::HandleEvents => temp::handle_events(),
                Msg::AppendChild { parent, child } => {
                    app.surface_service.append_child(parent.into(), child.into())
                }
                Msg::SetBackgroundColor { surface, color } => app.surface_service.set_background_color(surface.into(), color),
                _ => {}
            },
        }
    }
}

struct App {
    surface_service: SurfaceService,
    layout_service: YogaLayoutService,
    render_service: WebrenderRenderService
}

impl Into<Id> for SurfaceId {
    fn into(self) -> Id {
        self.0
    }
}
