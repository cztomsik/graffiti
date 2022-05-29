use super::app::{App, AppOwned};
use crossbeam_channel::{unbounded as channel, Receiver, Sender};
use graffiti_glfw::*;
use std::ffi::CString;
use std::os::raw::{c_double, c_int, c_uint, c_void};
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};

pub type WindowId = u32;

pub struct Window {
    _app: Arc<App>,
    glfw_window: AppOwned<GlfwWindow>,
    events: Receiver<Event>,

    // glfw does not provide getter
    title: Mutex<String>,
}

impl Window {
    pub fn new(title: &str, width: i32, height: i32) -> Arc<Self> {
        let app = App::current().expect("no App");
        let c_title = CString::new(title).unwrap();
        let (events_tx, events) = channel();

        let glfw_window = app.await_task(move || unsafe {
            glfwDefaultWindowHints();

            #[cfg(target_os = "macos")]
            {
                glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
                glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 2);
                glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GLFW_TRUE);
                glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
            }

            let glfw_window = glfwCreateWindow(width, height, c_title.as_ptr(), null_mut(), null_mut());
            assert_ne!(glfw_window, null_mut(), "create GLFW window");

            // Sender<Event>
            glfwSetWindowUserPointer(glfw_window, Box::into_raw(Box::new(events_tx)) as *mut _);

            glfwSetCursorPosCallback(glfw_window, handle_glfw_cursor_pos);
            glfwSetScrollCallback(glfw_window, handle_glfw_scroll);
            glfwSetMouseButtonCallback(glfw_window, handle_glfw_mouse_button);
            glfwSetCharCallback(glfw_window, handle_glfw_char);
            glfwSetKeyCallback(glfw_window, handle_glfw_key);
            glfwSetWindowSizeCallback(glfw_window, handle_glfw_window_size);
            glfwSetFramebufferSizeCallback(glfw_window, handle_glfw_framebuffer_size);
            glfwSetWindowCloseCallback(glfw_window, handle_glfw_window_close);

            // detach
            glfwMakeContextCurrent(std::ptr::null_mut());

            AppOwned(glfw_window)
        });

        Arc::new(Self {
            _app: app,
            title: Mutex::new(title.to_owned()),
            glfw_window,
            events,
        })
    }

    pub fn native_handle(&self) -> *mut c_void {
        #[cfg(target_os = "macos")]
        return self.glfw_window.with(|win| unsafe { glfwGetCocoaWindow(win) as usize }) as _;

        #[allow(unreachable_code)]
        std::ptr::null_mut()
    }

    pub fn title(&self) -> String {
        self.title.lock().unwrap().clone()
    }

    pub fn set_title(&self, title: &str) {
        *self.title.lock().unwrap() = title.to_owned();

        let title = CString::new(title).unwrap();
        self.glfw_window
            .with(move |win| unsafe { glfwSetWindowTitle(win, title.as_ptr()) });
    }

    pub fn resizable(&self) -> bool {
        self.glfw_window
            .with(|win| unsafe { glfwGetWindowAttrib(win, GLFW_RESIZABLE) == GLFW_TRUE })
    }

    pub fn set_resizable(&self, resizable: bool) {
        self.glfw_window
            .with(move |win| unsafe { glfwSetWindowAttrib(win, GLFW_RESIZABLE, resizable as _) });
    }

    pub fn size(&self) -> (i32, i32) {
        self.glfw_window.with(|win| {
            let mut size = (0, 0);
            unsafe { glfwGetWindowSize(win, &mut size.0, &mut size.1) }
            size
        })
    }

    pub fn set_size(&self, (width, height): (i32, i32)) {
        self.glfw_window
            .with(move |win| unsafe { glfwSetWindowSize(win, width as _, height as _) });
    }

    pub fn framebuffer_size(&self) -> (i32, i32) {
        self.glfw_window.with(|win| {
            let mut size = (0, 0);
            unsafe { glfwGetFramebufferSize(win, &mut size.0, &mut size.1) }
            size
        })
    }

    pub fn content_scale(&self) -> (f32, f32) {
        self.glfw_window.with(|win| {
            let mut scale = (0., 0.);
            unsafe { glfwGetWindowContentScale(win, &mut scale.0, &mut scale.1) }
            scale
        })
    }

    pub fn transparent(&self) -> bool {
        self.glfw_window
            .with(|win| unsafe { glfwGetWindowAttrib(win, GLFW_TRANSPARENT_FRAMEBUFFER) == GLFW_TRUE })
    }

    pub fn opacity(&self) -> f32 {
        self.glfw_window.with(|win| unsafe { glfwGetWindowOpacity(win) })
    }

    pub fn set_opacity(&self, opacity: f32) {
        self.glfw_window
            .with(move |win| unsafe { glfwSetWindowOpacity(win, opacity) });
    }

    pub fn visible(&self) -> bool {
        self.glfw_window
            .with(|win| unsafe { glfwGetWindowAttrib(win, GLFW_VISIBLE) == GLFW_TRUE })
    }

    pub fn show(&self) {
        self.glfw_window.with(|win| unsafe { glfwShowWindow(win) });
    }

    pub fn hide(&self) {
        self.glfw_window.with(|win| unsafe { glfwHideWindow(win) });
    }

    pub fn focused(&self) -> bool {
        self.glfw_window
            .with(|win| unsafe { glfwGetWindowAttrib(win, GLFW_FOCUSED) == GLFW_TRUE })
    }

    pub fn focus(&self) {
        self.glfw_window.with(|win| unsafe { glfwFocusWindow(win) });
    }

    pub fn minimized(&self) -> bool {
        self.glfw_window
            .with(|win| unsafe { glfwGetWindowAttrib(win, GLFW_ICONIFIED) == GLFW_TRUE })
    }

    pub fn minimize(&self) {
        self.glfw_window.with(|win| unsafe { glfwIconifyWindow(win) });
    }

    pub fn maximized(&self) -> bool {
        self.glfw_window
            .with(|win| unsafe { glfwGetWindowAttrib(win, GLFW_MAXIMIZED) == GLFW_TRUE })
    }

    pub fn maximize(&self) {
        self.glfw_window.with(|win| unsafe { glfwMaximizeWindow(win) });
    }

    pub fn restore(&self) {
        self.glfw_window.with(|win| unsafe { glfwRestoreWindow(win) });
    }

    pub fn request_attention(&self) {
        self.glfw_window.with(|win| unsafe { glfwRequestWindowAttention(win) });
    }

    // event loop

    pub fn should_close(&self) -> bool {
        self.glfw_window
            .with(|win| unsafe { glfwWindowShouldClose(win) == GLFW_TRUE })
    }

    pub fn set_should_close(&self, value: bool) {
        self.glfw_window
            .with(move |win| unsafe { glfwSetWindowShouldClose(win, value as _) });
    }

    // note it needs to be processed one by one because each event can cause new changes,
    // styles, dimensions and so the target might not be valid anymore
    pub fn events(&self) -> &Receiver<Event> {
        &self.events
    }

    // GL

    pub unsafe fn make_current(&self) {
        glfwMakeContextCurrent(self.glfw_window.0);
    }

    pub unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
        // TODO: this is magic we should rather panic if not current
        self.make_current();

        let symbol = CString::new(symbol).unwrap();
        glfwGetProcAddress(symbol.as_ptr())
    }

    // GLFW says it's possible to call this from any thread but
    // some people say everything related to HDC is tied to the original thread
    // and drivers are free to depend on this
    pub fn swap_buffers(&self) {
        self.glfw_window.with(|win| unsafe { glfwSwapBuffers(win) });
    }

    pub fn clipboard_string(&self) -> Option<String> {
        todo!()
        // TODO: we should copy and we also need to check for null
        // unsafe { CStr::from_ptr(glfwGetClipboardString(self.glfw_window)).to_str().ok() }
    }

    pub fn set_clipboard_string(&self, string: &str) {
        let string = CString::new(string).unwrap();
        self.glfw_window
            .with(move |win| unsafe { glfwSetClipboardString(win, string.as_ptr()) });
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.glfw_window.with(|win| unsafe {
            let ptr = glfwGetWindowUserPointer(win);
            glfwSetWindowUserPointer(win, null_mut());
            glfwDestroyWindow(win);

            drop(Box::from_raw(ptr as *mut Sender<Event>));
        });
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, u32)]
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

unsafe extern "C" fn handle_glfw_cursor_pos(w: GlfwWindow, x: c_double, y: c_double) {
    send_event(w, Event::CursorPos(x as _, y as _));
}

unsafe extern "C" fn handle_glfw_scroll(w: GlfwWindow, x: c_double, y: c_double) {
    send_event(w, Event::Scroll(x as _, y as _));
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
    send_event(w, Event::KeyPress(char));
}

unsafe extern "C" fn handle_glfw_window_size(w: GlfwWindow, width: c_int, height: c_int) {
    send_event(w, Event::Resize(width as _, height as _));
}

unsafe extern "C" fn handle_glfw_framebuffer_size(w: GlfwWindow, width: c_int, height: c_int) {
    send_event(w, Event::FramebufferSize(width as _, height as _));
}

unsafe extern "C" fn handle_glfw_window_close(w: GlfwWindow) {
    send_event(w, Event::Close);
}

unsafe fn send_event(win: GlfwWindow, event: Event) {
    let sender = &*(glfwGetWindowUserPointer(win) as *const Sender<_>);

    sender.send(event).unwrap();
}

// from GLFW to JS `event.which`
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
