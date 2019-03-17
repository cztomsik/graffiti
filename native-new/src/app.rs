use crate::api::{App, AppEvent, WindowEvent, WindowId};
use crate::window::GlutinWindow;
use glutin::{ContextBuilder, ContextTrait, ControlFlow, EventsLoop, WindowBuilder};
use std::collections::BTreeMap;

// TODO: it's not wrong technically, but it might be confusing name (GlutinWindow too)
pub struct GlutinApp {
    events_loop: EventsLoop,
    windows: BTreeMap<WindowId, GlutinWindow>,
    native_ids: BTreeMap<glutin::WindowId, WindowId>,
    next_window_id: WindowId
}

impl GlutinApp {
    pub fn new() -> Self {
        let events_loop = EventsLoop::new();

        GlutinApp {
            events_loop,
            windows: BTreeMap::new(),
            native_ids: BTreeMap::new(),
            next_window_id: 1
        }
    }
}

impl App<GlutinWindow> for GlutinApp {
    fn get_next_event(&mut self) -> Option<AppEvent> {
        let mut result = None;

        let GlutinApp { events_loop, windows, native_ids, .. } = self;

        // weird but necessary (we want to keep the control)
        events_loop.run_forever(|e| match e {
            glutin::Event::Awakened => ControlFlow::Break,
            glutin::Event::WindowEvent { window_id, event } => {
                let id = native_ids.get(&window_id).expect("got message for nonexistent window");
                let window = windows.get(&id).unwrap();

                result = Some(AppEvent::WindowEvent {
                    window: *id,
                    event: window.translate_event(event),
                });

                ControlFlow::Break
            }
            _ => ControlFlow::Continue,
        });

        result
    }

    fn create_window(&mut self) -> WindowId {
        let window_builder = WindowBuilder::new();
        let glutin_context = ContextBuilder::new()
            .build_windowed(window_builder, &self.events_loop)
            .expect("couldn't create gl context");

        unsafe {
            glutin_context
                .make_current()
                .expect("couldn't bind gl context with the current thread");
        }

        let window = GlutinWindow::new(glutin_context);
        let id = self.next_window_id;
        let native_id = window.id();

        self.windows.insert(id, window);
        self.native_ids.insert(native_id, id);

        self.next_window_id = self.next_window_id + 1;

        id
    }

    fn get_window(&mut self, id: WindowId) -> &mut GlutinWindow {
        self.windows.get_mut(&id).expect("window not found")
    }

    fn destroy_window(&mut self, id: WindowId) {
        let native_id = self.get_window(id).id();

        self.windows.remove(&id);
        self.native_ids.remove(&native_id);
    }
}
