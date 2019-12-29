use crate::commons::{SurfaceId, Bounds};
use crate::app::{TheApp, WindowId};
use crate::window::{SceneChange, Event};

#[derive(Debug, Clone)]
pub enum ApiMsg {
    // sorted by whats most common
    GetEvents { poll: bool },
    UpdateScene { window: WindowId, changes: Vec<SceneChange> },
    GetBounds { window: WindowId, surface: SurfaceId },
    CreateWindow { title: String, width: i32, height: i32 },
    ResizeWindow { window: WindowId },
    DestroyWindow { window: WindowId },
}

#[derive(Debug, Clone)]
pub enum ApiResponse {
    Events { events: Vec<Event> },
    Nothing {},
    Bounds { bounds: Bounds }
}

pub unsafe fn init_api() -> Api {
    if INIT_CALLED {
        panic!("Already initialized")
    } else {
        INIT_CALLED = true;
    }

    Api { app: TheApp::init() }
}

static mut INIT_CALLED: bool = false;

pub struct Api {
    app: TheApp
}

impl Api {
    pub fn send(&mut self, msg: ApiMsg) -> ApiResponse {
        use ApiMsg::*;
        use ApiResponse::*;

        let Api { app, .. } = self;

        match msg {
            CreateWindow { title, width, height } => { app.create_window(&title, width, height); Nothing {} }
            GetEvents { poll } => Events { events: app.get_events(poll) },
            UpdateScene { window, changes } => { app.update_window_scene(window, &changes); Nothing {} }
            GetBounds { window, surface } => Bounds { bounds: app.get_bounds(window, surface) },
            _ => unimplemented!(),
        }
    }
}
