use crate::api::{App, AppEvent, WindowEvent};
use crate::window::GlutinWindow;
use glutin::{ContextBuilder, ContextTrait, ControlFlow, EventsLoop, WindowBuilder, WindowId};
use std::collections::BTreeMap;

pub struct GlutinApp {
    events_loop: EventsLoop,
    windows: BTreeMap<WindowId, GlutinWindow>,
}

impl GlutinApp {
    pub fn new() -> Self {
        let events_loop = EventsLoop::new();

        GlutinApp {
            events_loop,
            windows: BTreeMap::new(),
        }
    }

    // temp
    pub fn get_first_window_id(&self) -> WindowId {
        let ids: Vec<&WindowId> = self.windows.keys().into_iter().collect();

        ids[0].clone()
    }
}

impl App<GlutinWindow> for GlutinApp {
    fn get_next_event(&mut self) -> Option<AppEvent> {
        let mut result = None;

        let GlutinApp { events_loop, windows } = self;

        // weird but necessary (we want to keep the control)
        events_loop.run_forever(|e| match e {
            glutin::Event::Awakened => ControlFlow::Break,
            glutin::Event::WindowEvent { window_id, event } => {
                let window = windows.get(&window_id).expect("got message for nonexistent window");

                result = Some(AppEvent::WindowEvent {
                    window: window_id,
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
        let id = window.id();

        self.windows.insert(id, window);

        id
    }

    fn get_window(&mut self, id: WindowId) -> &mut GlutinWindow {
        self.windows.get_mut(&id).expect("window not found")
    }

    fn destroy_window(&mut self, id: WindowId) {
        self.windows.remove(&id);
    }
}
