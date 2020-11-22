use crate::document::{Document, NodeId};
use crate::gfx::Surface;
use crate::viewport::Viewport;
use std::sync::mpsc::Receiver;

// high-level event with target
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum Event {
    MouseMove { target: NodeId, x: f64, y: f64 },
    MouseDown { target: NodeId },
    MouseUp { target: NodeId },
    Scroll { target: NodeId, x: f64, y: f64 },
    KeyDown { which: u32 },
    KeyUp { which: u32 },
    KeyPress { char: char },

    Resize { width: i32, height: i32 },
    Close,
}

// it *could* have more state but everything flows (with events) to JS anyway
// and we want to keep API short so it's probably not worth it
pub struct Window {
    events_rx: Receiver<WindowEvent>,
    mouse_pos: (f64, f64),
    viewport: Viewport,
}

impl Window {
    pub(crate) fn new(events_rx: Receiver<WindowEvent>, surface: Surface) -> Self {
        Self {
            events_rx,
            mouse_pos: (0., 0.),

            viewport: Viewport::new(surface),
        }
    }

    pub fn document(&self) -> &Document {
        self.viewport.document()
    }

    pub fn document_mut(&mut self) -> &mut Document {
        self.viewport.document_mut()
    }

    pub fn update(&mut self) {
        self.viewport.update();
    }

    pub fn render(&mut self) {
        self.viewport.render();
    }

    // needs to be processed one by one because each event can cause new changes,
    // styles, dimensions and so the target might not be valid anymore
    pub fn take_event(&mut self) -> Option<Event> {
        let (x, y) = self.mouse_pos;
        let target = self.viewport.node_at_pos((x as _, y as _));

        match self.events_rx.try_recv() {
            Ok(ev) => Some(match ev {
                WindowEvent::CursorPos(x, y) => {
                    self.mouse_pos = (x, y);

                    Event::MouseMove { target, x, y }
                }

                WindowEvent::MouseDown => Event::MouseDown { target },
                WindowEvent::MouseUp => Event::MouseUp { target },
                WindowEvent::Scroll(x, y) => Event::Scroll { target, x, y },

                WindowEvent::KeyDown(which) => Event::KeyDown { which },
                WindowEvent::KeyUp(which) => Event::KeyUp { which },
                WindowEvent::Char(char) => Event::KeyPress { char },

                WindowEvent::Resize(width, height) => Event::Resize { width, height },
                WindowEvent::Close => Event::Close,
            }),
            _ => None,
        }
    }
}

// low-level (system) event
#[derive(Debug)]
pub(crate) enum WindowEvent {
    CursorPos(f64, f64),
    MouseDown,
    MouseUp,
    Scroll(f64, f64),

    // JS e.which
    KeyUp(u32),
    KeyDown(u32),
    Char(char),

    Resize(i32, i32),
    Close,
}
