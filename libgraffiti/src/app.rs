use crate::commons::{ElementId, Bounds};
use crate::viewport::{Viewport, Event, SceneChange};
use crate::platform;
use crate::platform::{NativeWindow, WINDOWS_PTR, VIEWPORTS_PTR, PENDING_EVENTS_PTR};



/// Root for the whole native part
/// Only one instance is allowed
///
/// - create/destroy windows
/// - update their viewports
/// - get pending events (with resolved targets) of all windows
pub struct App {
    // there's not many windows so it should be fine
    windows: Vec<NativeWindow>,
    viewports: Vec<Viewport>,

    //async_updates: Sender<AsyncUpdate>,
}

//struct AsyncUpdate(WindowId, NativeWindow, Vec<SceneChange>);
//unsafe impl std::marker::Send for AsyncUpdate {}

pub type WindowId = usize;

#[derive(Debug, Clone)]
pub struct WindowEvent {
    pub window: WindowId,
    pub event: Event,
}

impl App {
    pub unsafe fn init() -> Self {
        platform::init();

        /* worker poc
        let viewports: Vec<Viewport> = Vec::new();
        let viewports = Arc::new(Mutex::new(viewports));

        // worker
        let (async_updates, rx) = channel();
        let worker_viewports = viewports.clone();
        std::thread::spawn(move || {
            loop {
                let AsyncUpdate(id, native_window, updates) = rx.recv().unwrap();

                unsafe { platform::make_current(native_window) }

                println!("update");
                worker_viewports.lock().unwrap().deref_mut()[id].update_styles(&updates);

                unsafe {
                    println!("swap");
                    platform::swap_buffers(native_window)
                }
           }
        });
        */

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
        let viewport = Viewport::new((width as f32, height as f32));

        // detach so it can be attached by another thread
        unsafe { platform::detach_current() }

        self.windows.push(native_window);
        self.viewports.push(viewport);

        id
    }

    pub fn update_window_scene(&mut self, id: WindowId, changes: &[SceneChange]) {
        let native_window = &mut self.windows[id];
        let viewport = &mut self.viewports[id];

        unsafe { platform::make_current(*native_window) }

        viewport.update_scene(changes);
        unsafe {
            platform::swap_buffers(*native_window)
        }
    }

    pub fn get_offset_bounds(&self, window: WindowId, element: ElementId) -> Bounds {
        self.viewports[window].get_offset_bounds(element)
    }

    pub fn destroy_window(&mut self, _id: WindowId) {
        todo!()

        // TODO (+freelist)
        //   free viewports[id]
        //   platform::destroy_window(native_windows[id])
    }
}


