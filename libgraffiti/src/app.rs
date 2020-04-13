use crate::commons::Bounds;
use crate::platform;
use crate::platform::{NativeWindow, PENDING_EVENTS_PTR, VIEWPORTS_PTR, WINDOWS_PTR};
use crate::render::backend::gl::GlRenderBackend;
use crate::viewport::{Event, GlViewport, NodeId};

/// Root for the whole native part
/// Only one instance is allowed
///
/// - create/destroy windows
/// - update their viewports
/// - get pending events (with resolved targets) of all windows
pub struct App {
    // there's not many windows so it should be fine
    windows: Vec<NativeWindow>,
    viewports: Vec<GlViewport>,
}

pub type WindowId = usize;

#[derive(Debug, Clone)]
pub struct WindowEvent {
    pub window: WindowId,
    pub event: Event,
}

impl App {
    pub unsafe fn init() -> Self {
        platform::init();

        App {
            windows: Vec::new(),
            viewports: Vec::new(),
        }
    }
}

impl App {
    pub fn get_events(&mut self, poll: bool) -> Vec<WindowEvent> {
        let mut events = Vec::new();

        unsafe {
            WINDOWS_PTR = &mut self.windows;
            VIEWPORTS_PTR = &mut self.viewports;
            PENDING_EVENTS_PTR = &mut events;

            platform::get_events(poll);

            WINDOWS_PTR = std::ptr::null_mut();
            VIEWPORTS_PTR = std::ptr::null_mut();
            PENDING_EVENTS_PTR = std::ptr::null_mut();
        }

        events
    }

    pub fn create_window(&mut self, title: &str, width: i32, height: i32) -> WindowId {
        let id = self.windows.len();

        let native_window = unsafe { platform::create_window(title, width, height) };
        let viewport = GlViewport::new(GlRenderBackend::new(), (width as f32, height as f32));

        // detach so it can be attached by another thread
        unsafe { platform::detach_current() }

        self.windows.push(native_window);
        self.viewports.push(viewport);

        id
    }

    pub fn update_window_scene(&mut self, id: WindowId, f: &mut impl FnMut(&mut GlViewport)) {
        let native_window = &mut self.windows[id];
        let viewport = &mut self.viewports[id];

        unsafe { platform::make_current(*native_window) }

        f(viewport);
        viewport.update();

        unsafe { platform::swap_buffers(*native_window) }
    }

    pub fn get_offset_bounds(&self, window: WindowId, element: NodeId) -> Bounds {
        self.viewports[window].get_offset_bounds(element)
    }

    pub fn destroy_window(&mut self, _id: WindowId) {
        todo!()

        // TODO (+freelist)
        //   free viewports[id]
        //   platform::destroy_window(native_windows[id])
    }
}
