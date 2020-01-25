use crate::commons::{ElementId, Bounds};
use crate::app::{App, WindowId, WindowEvent};
use crate::viewport::{SceneChange};

// all of the js <-> native communication goes through this  
// single endpoint using one tagged union for the message and
// one for the respective response. thanks to that we can
// auto-generate most of the bindings
trait InteropApi {
    fn send(&mut self, msg: AppMsg) -> AppResponse;
}

// one variant for anything what needs (sync) return value
#[derive(Debug, Clone)]
pub enum AppMsg {
    GetEvents { poll: bool },
    CreateWindow { title: String, width: i32, height: i32 },
    ResizeWindow { window: WindowId, width: i32, height: i32 },
    UpdateScene { window: WindowId, changes: Vec<SceneChange> },
    GetOffsetBounds { window: WindowId, element: ElementId },
    DestroyWindow { window: WindowId },
}

// what you can get in return (sync)
#[derive(Debug, Clone)]
pub enum AppResponse {
    WindowId { id: WindowId },
    Ack {},
    Events { events: Vec<WindowEvent> },
    Bounds { bounds: Bounds },
}

impl InteropApi for App {
    fn send(&mut self, msg: AppMsg) -> AppResponse {
        use AppMsg::*;
        use AppResponse::*;

        //println!("{:?}", &msg);

        // sorted by whats most common (perf)
        match msg {
            GetEvents { poll } => Events { events: self.get_events(poll) },
            UpdateScene { window, changes } => { self.update_window_scene(window, &changes); Ack {} }
            GetOffsetBounds { window, element } => Bounds { bounds: self.get_offset_bounds(window, element) },
            CreateWindow { title, width, height } => { WindowId { id: self.create_window(&title, width, height) } }
            ResizeWindow { .. } => todo!(),
            DestroyWindow { window } => { self.destroy_window(window); Ack {} },
        }
    }
}

/// Some of the data can be exchanged with nodejs
/// and/or other platforms
///
/// Each platform should implement this for basic types
/// and then provide `interop!` macro & include the contents
/// of `./generated.rs` which in turn calls this macro
/// to generate structs, enums & tagged unions
pub trait Interop<T> {
    fn from_external(external: T) -> Self;
    fn to_external(self) -> T;
}

#[cfg(not(target_arch = "wasm32"))]
mod nodejs;

// TODO: wasm
