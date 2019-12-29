use crate::platform::{WINDOWS_PTR, PENDING_EVENTS_PTR};
use crate::commons::{SurfaceId, Bounds};
use crate::viewport::{Viewport, Event, SceneChange};
use std::collections::BTreeMap;
use std::ptr;
use crate::platform;
use crate::platform::{NativeWindow};

/// Root for the whole native part
/// Only one instance is allowed
///
/// - create/destroy windows
/// - update their viewports
/// - get pending events (with surface targets) of all windows
pub struct App {
    // primary storage
    //
    // keyed by NativeWimdow so that we can find viewport quickly
    // in native event handlers
    window_viewports: BTreeMap<NativeWindow, Viewport>,

    // quickly get to a window/viewport using WindowId
    native_windows: Vec<NativeWindow>,
    //viewports: Vec<&'a mut Viewport>,
}

pub type WindowId = usize;

impl App {
    pub unsafe fn init() -> Self {
        platform::init();

        if INIT_CALLED {
            panic!("Already initialized")
        } else {
            INIT_CALLED = true;
        }

        App {
            window_viewports: BTreeMap::new(),
            //viewports: Vec::new(),
            native_windows: Vec::new(),
        }
    }
}

static mut INIT_CALLED: bool = false;

impl App {
    pub fn get_events(&mut self, poll: bool) -> Vec<Event> {
        // TODO: share the vec, clear it only
        // maybe it can be part of App state?
        let mut events = Vec::new();

        unsafe {
            WINDOWS_PTR = &mut self.window_viewports;
            PENDING_EVENTS_PTR = &mut events;

            platform::get_events(poll);

            PENDING_EVENTS_PTR = ptr::null_mut();
        }

        events
    }

    pub fn create_window(&mut self, title: &str, width: i32, height: i32) -> WindowId {
        let id = self.native_windows.len();

        let native_window = unsafe { platform::create_window(title, width, height) };
        let viewport = Viewport::new(width, height);

        self.native_windows.push(native_window);
        self.window_viewports.insert(native_window, viewport);

        id
    }

    pub fn update_window_scene(&mut self, id: WindowId, changes: &[SceneChange]) {
        let native_window = self.native_windows[id];
        let viewport = self.window_viewports.get_mut(&native_window).expect("window not found");

        viewport.update_scene(changes);
        unsafe {
            platform::swap_buffers(native_window)
        }
    }

    pub fn get_bounds(&self, window: WindowId, surface: SurfaceId) -> Bounds {
        let native_window = self.native_windows[window];
        let viewport = self.window_viewports.get(&native_window).expect("window not found");

        viewport.get_bounds(surface)
    }

    pub fn destroy_window(&mut self, _id: WindowId) {
        // TODO
        //   free viewports[id]
        //   platform::destroy_window(native_windows[id])
    }
}


