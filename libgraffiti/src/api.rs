use crate::app::{TheApp, WindowId};
use crate::window::{SceneChange, Event};

#[derive(Debug)]
pub enum ApiMsg {
    CreateWindow { width: i32, height: i32 },
    GetEvents { poll: bool },
    UpdateScene { window: WindowId, changes: Vec<SceneChange> },
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
    pub fn send(&mut self, msg: ApiMsg) -> Option<Vec<Event>> {
        use ApiMsg::*;

        let Api { app, .. } = self;

        match msg {
            CreateWindow { width, height } => { app.create_window(width, height); None }
            GetEvents { poll } => { Some(app.get_events(poll)) }
            UpdateScene { window, changes } => { app.update_window_scene(window, &changes); None }
        }
    }
}
