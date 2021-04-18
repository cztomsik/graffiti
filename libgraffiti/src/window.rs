use super::App;
use graffiti_glfw::*;
use std::ffi::CStr;
use std::os::raw::{c_double, c_int, c_uint, c_void};
use std::ptr::null_mut;
use std::rc::Rc;
use crossbeam_channel::{unbounded as channel, Receiver, Sender};

pub struct Window {
    _app: Rc<App>,
    title: String,
    glfw_window: GlfwWindow,
    events: Receiver<Event>,
}

impl Window {
    pub fn new(app: &Rc<App>, title: &str, width: i32, height: i32) -> Self {
        unsafe {
            glfwDefaultWindowHints();

            #[cfg(target_os = "macos")]
            {
                glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
                glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 2);
                glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GLFW_TRUE);
                glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
            }

            let glfw_window = glfwCreateWindow(width, height, *c_str!(title), null_mut(), null_mut());
            assert_ne!(glfw_window, null_mut(), "create GLFW window");
            let (events_tx, events) = channel();

            // TODO: drop
            glfwSetWindowUserPointer(glfw_window, Box::into_raw(Box::new(events_tx)) as *mut _);

            glfwSetCursorPosCallback(glfw_window, handle_glfw_cursor_pos);
            glfwSetScrollCallback(glfw_window, handle_glfw_scroll);
            glfwSetMouseButtonCallback(glfw_window, handle_glfw_mouse_button);
            glfwSetCharCallback(glfw_window, handle_glfw_char);
            glfwSetKeyCallback(glfw_window, handle_glfw_key);
            glfwSetWindowSizeCallback(glfw_window, handle_glfw_window_size);
            glfwSetFramebufferSizeCallback(glfw_window, handle_glfw_framebuffer_size);
            glfwSetWindowCloseCallback(glfw_window, handle_glfw_window_close);

            Self {
                _app: Rc::clone(app),
                title: title.to_owned(),
                glfw_window,
                events,
            }
        }
    }

    #[cfg(target_os = "macos")]
    pub fn native_handle(&mut self) -> *mut c_void {
        unsafe { glfwGetCocoaWindow(self.glfw_window) as _ }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: &str) {
        unsafe { glfwSetWindowTitle(self.glfw_window, *c_str!(title)) }

        self.title = title.to_owned();
    }

    pub fn resizable(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.glfw_window, GLFW_RESIZABLE) == GLFW_TRUE }
    }

    pub fn set_resizable(&mut self, resizable: bool) {
        unsafe { glfwSetWindowAttrib(self.glfw_window, GLFW_RESIZABLE, resizable as _) }
    }

    pub fn size(&self) -> (i32, i32) {
        let mut size = (0, 0);

        unsafe { glfwGetWindowSize(self.glfw_window, &mut size.0, &mut size.1) }

        size
    }

    pub fn set_size(&mut self, (width, height): (i32, i32)) {
        unsafe { glfwSetWindowSize(self.glfw_window, width as _, height as _) }
    }

    pub fn framebuffer_size(&self) -> (i32, i32) {
        let mut size = (0, 0);

        unsafe { glfwGetFramebufferSize(self.glfw_window, &mut size.0, &mut size.1) }

        size
    }

    pub fn content_scale(&self) -> (f32, f32) {
        let mut scale = (0., 0.);

        unsafe { glfwGetWindowContentScale(self.glfw_window, &mut scale.0, &mut scale.1) }

        scale
    }

    pub fn transparent(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.glfw_window, GLFW_TRANSPARENT_FRAMEBUFFER) == GLFW_TRUE }
    }

    pub fn opacity(&self) -> f32 {
        unsafe { glfwGetWindowOpacity(self.glfw_window) }
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        unsafe { glfwSetWindowOpacity(self.glfw_window, opacity) }
    }

    pub fn visible(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.glfw_window, GLFW_VISIBLE) == GLFW_TRUE }
    }

    pub fn show(&mut self) {
        unsafe { glfwShowWindow(self.glfw_window) }
    }

    pub fn hide(&mut self) {
        unsafe { glfwHideWindow(self.glfw_window) }
    }

    pub fn focused(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.glfw_window, GLFW_FOCUSED) == GLFW_TRUE }
    }

    pub fn focus(&mut self) {
        unsafe { glfwFocusWindow(self.glfw_window) }
    }

    pub fn minimized(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.glfw_window, GLFW_ICONIFIED) == GLFW_TRUE }
    }

    pub fn minimize(&mut self) {
        unsafe { glfwIconifyWindow(self.glfw_window) }
    }

    pub fn maximized(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.glfw_window, GLFW_MAXIMIZED) == GLFW_TRUE }
    }

    pub fn maximize(&mut self) {
        unsafe { glfwMaximizeWindow(self.glfw_window) }
    }

    pub fn restore(&mut self) {
        unsafe { glfwRestoreWindow(self.glfw_window) }
    }

    pub fn request_attention(&mut self) {
        unsafe { glfwRequestWindowAttention(self.glfw_window) }
    }

    // event loop

    pub fn should_close(&self) -> bool {
        unsafe { glfwWindowShouldClose(self.glfw_window) == GLFW_TRUE }
    }

    pub fn set_should_close(&mut self, value: bool) {
        unsafe { glfwSetWindowShouldClose(self.glfw_window, value as _) }
    }

    // note it needs to be processed one by one because each event can cause new changes,
    // styles, dimensions and so the target might not be valid anymore
    pub fn events(&mut self) ->&Receiver<Event> {
        &self.events
    }

    // GL

    pub unsafe fn make_current(&mut self) {
        glfwMakeContextCurrent(self.glfw_window);
    }

    pub unsafe fn get_proc_address(&mut self, symbol: &str) -> *const c_void {
        // TODO: this is magic we should rather panic if not current
        self.make_current();

        glfwGetProcAddress(*c_str!(symbol))
    }

    // GLFW says it's possible to call this from any thread but
    // some people say everything related to HDC is tied to the original thread
    // and drivers are free to depend on this
    pub fn swap_buffers(&mut self) {
        unsafe { glfwSwapBuffers(self.glfw_window) }
    }

    pub fn clipboard_string(&self) -> Option<&str> {
        unsafe { CStr::from_ptr(glfwGetClipboardString(self.glfw_window)).to_str().ok() }
    }

    pub fn set_clipboard_string(&mut self, string: &str) {
        unsafe { glfwSetClipboardString(self.glfw_window, *c_str!(string)) }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            let ptr = glfwGetWindowUserPointer(self.glfw_window);
            glfwSetWindowUserPointer(self.glfw_window, null_mut());
            glfwDestroyWindow(self.glfw_window);

            drop(Box::from_raw(ptr as *mut Sender<Event>));
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Event {
    CursorPos(f64, f64),
    MouseDown,
    MouseUp,
    Scroll(f64, f64),

    // JS e.which
    KeyUp(u32),
    KeyDown(u32),
    Char(char),

    Resize(i32, i32),
    FramebufferSize(i32, i32),
    Close,
}

unsafe extern "C" fn handle_glfw_cursor_pos(w: GlfwWindow, x: c_double, y: c_double) {
    send_event(w, Event::CursorPos(x, y));
}

unsafe extern "C" fn handle_glfw_scroll(w: GlfwWindow, x: c_double, y: c_double) {
    send_event(w, Event::Scroll(x, y));
}

unsafe extern "C" fn handle_glfw_mouse_button(w: GlfwWindow, _button: c_int, action: c_int, _mods: c_int) {
    send_event(
        w,
        match action {
            GLFW_PRESS => Event::MouseDown,
            GLFW_RELEASE => Event::MouseUp,
            _ => unreachable!("press/release expected"),
        },
    );
}

unsafe extern "C" fn handle_glfw_key(w: GlfwWindow, key: c_int, _scancode: c_int, action: c_int, _mods: c_int) {
    let which = key_code(key);

    send_event(
        w,
        match action {
            // TODO: repeat works for some keys but for some it doesn't
            // not sure if it's specific for mac (special chars overlay)
            GLFW_RELEASE => Event::KeyUp(which),
            _ => Event::KeyDown(which),
        },
    );
}

unsafe extern "C" fn handle_glfw_char(w: GlfwWindow, char: c_uint) {
    send_event(w, Event::Char(std::char::from_u32_unchecked(char)));
}

unsafe extern "C" fn handle_glfw_window_size(w: GlfwWindow, width: c_int, height: c_int) {
    send_event(w, Event::Resize(width, height));
}

unsafe extern "C" fn handle_glfw_framebuffer_size(w: GlfwWindow, width: c_int, height: c_int) {
    send_event(w, Event::FramebufferSize(width, height));
}

unsafe extern "C" fn handle_glfw_window_close(w: GlfwWindow) {
    send_event(w, Event::Close);
}

unsafe fn send_event(win: GlfwWindow, event: Event) {
    let sender = &*(glfwGetWindowUserPointer(win) as *const Sender<_>);

    sender.send(event).unwrap();
}

// from glfw to js `e.which`
// TODO: modifier (left/right for shift/ctrl/alt)
fn key_code(key: c_int) -> u32 {
    (match key {
        // some codes are the same
        32 | 48..=57 | 65..=90 => key,
        91..=93 => key + 128,

        44 => 188,
        45 => 187,
        46 => 190,
        47 => 189,
        59 => 186,
        61 => 187,
        256 => 27,
        257 => 13,
        258 => 9,
        259 => 8,
        262 => 39,
        263 => 37,
        264 => 40,
        265 => 38,

        // TODO: -1, APOSTROPHE, GRAVE_ACCENT, WORLD1, WORLD2, INSERT, DELETE,
        //   PAGE_UP, PAGE_DOWN, HOME, END, CAPS_LOCK, SCROLL_LOCK, NUM_LOCK
        //   PRINT_SCREEN, PAUSE, F1-F25, KP0-KP9, KP_*
        //   LEFT/RIGHT_SHIFT/CONTROL/ALT
        _ => 0,
    }) as u32
}
