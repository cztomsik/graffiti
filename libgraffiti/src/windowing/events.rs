use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    CursorPos(f32, f32),
    MouseDown,
    MouseUp,
    Scroll(f32, f32),

    // JS e.which
    KeyUp(u32),
    KeyDown(u32),
    KeyPress(u32),

    Resize(f32, f32),
    FramebufferSize(f32, f32),
    Close,
}

pub trait EventHandler: Send {
    fn handle_event(&mut self, event: Event);
}

impl<F: FnMut(Event) + Send> EventHandler for F {
    fn handle_event(&mut self, event: Event) {
        self(event)
    }
}

impl EventHandler for () {
    fn handle_event(&mut self, event: Event) {}
}

impl fmt::Debug for dyn EventHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("dyn EventHandler").finish()
    }
}
