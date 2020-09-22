use crate::gfx::Surface;
use crate::window::{Window, WindowEvent};
use std::sync::mpsc::{channel, Sender};

use std::os::raw::{c_char, c_double, c_int, c_uint, c_void};
use std::ptr::null_mut;

pub struct App {
    // so it's !Send + !Sync
//_marker: *const (),
}

impl App {
    // TODO: main thread, singleton
    pub fn new() -> Self {
        unsafe {
            graffiti_glfw::glfwInit();
            graffiti_glfw::glfwSetErrorCallback(handle_glfw_error);
        }

        Self {
            //_marker: std::ptr::null()
        }
    }

    pub fn create_window(&mut self, title: &str, width: i32, height: i32, notify: Box<dyn Fn() -> ()>) -> Window {
        let (events_tx, events_rx) = channel();

        // TODO: main thread
        let (_glfw_window, surface) = unsafe {
            use graffiti_glfw::*;

            glfwDefaultWindowHints();

            #[cfg(target_os = "macos")]
            {
                // TODO: doesn't work with nanovg
                //glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
                //glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 2);
                //glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GLFW_TRUE);
                //glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
            }

            let w = glfwCreateWindow(width, height, c_str!(title), null_mut(), null_mut());
            assert_ne!(w, std::ptr::null_mut(), "create GLFW window");

            // TODO: drop
            glfwSetWindowUserPointer(w, Box::into_raw(Box::new(WinCtx { events_tx, notify })) as *mut _);

            glfwSetCursorPosCallback(w, handle_glfw_cursor_pos);
            glfwSetScrollCallback(w, handle_glfw_scroll);
            glfwSetMouseButtonCallback(w, handle_glfw_mouse_button);
            glfwSetCharCallback(w, handle_glfw_char);
            glfwSetKeyCallback(w, handle_glfw_key);
            glfwSetWindowSizeCallback(w, handle_glfw_window_size);
            glfwSetFramebufferSizeCallback(w, handle_glfw_framebuffer_size);
            glfwSetWindowCloseCallback(w, handle_glfw_window_close);

            // attach
            glfwMakeContextCurrent(w);

            // context must be current here
            gl::load_with(|s| glfwGetProcAddress(c_str!(s)));

            let surface = Surface::new(w, (width as f32, height as f32));

            // VSYNC=0 to disable
            let vsync = std::env::var("VSYNC").map(|s| s.parse().expect("vsync number")).unwrap_or(1);
            glfwSwapInterval(vsync);

            // detach
            glfwMakeContextCurrent(null_mut());

            (w, surface)
        };

        Window::new(events_rx, surface)
    }

    // run main_thread tasks & wait for events
    //
    // TODO: check if main thread?
    pub fn tick(&mut self) {
        // TODO: main_thread_queue

        // TODO: opt_timeout
        unsafe { graffiti_glfw::glfwWaitEventsTimeout(0.5) }
    }

    // should be possible to call from anywhere
    pub fn wakeup(&self) {
        unsafe { graffiti_glfw::glfwPostEmptyEvent() }
    }
}

struct WinCtx {
    events_tx: Sender<WindowEvent>,
    notify: Box<dyn Fn() -> ()>,
}

use graffiti_glfw::{GlfwWindow, GLFW_PRESS, GLFW_RELEASE};

unsafe extern "C" fn handle_glfw_error(code: c_int, desc: *const c_char) {
    eprintln!("GLFW error {} {:?}", code, std::ffi::CStr::from_ptr(desc));
}

unsafe extern "C" fn handle_glfw_cursor_pos(w: GlfwWindow, x: c_double, y: c_double) {
    send_event(w, WindowEvent::CursorPos(x, y));
}

unsafe extern "C" fn handle_glfw_scroll(w: GlfwWindow, x: c_double, y: c_double) {
    send_event(w, WindowEvent::Scroll(x, y));
}

unsafe extern "C" fn handle_glfw_mouse_button(w: GlfwWindow, _button: c_int, action: c_int, _mods: c_int) {
    send_event(
        w,
        match action {
            GLFW_PRESS => WindowEvent::MouseDown,
            GLFW_RELEASE => WindowEvent::MouseUp,
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
            GLFW_RELEASE => WindowEvent::KeyUp(which),
            _ => WindowEvent::KeyDown(which),
        },
    );
}

unsafe extern "C" fn handle_glfw_char(w: GlfwWindow, char: c_uint) {
    send_event(w, WindowEvent::Char(std::char::from_u32_unchecked(char)));
}

unsafe extern "C" fn handle_glfw_window_size(w: GlfwWindow, width: c_int, height: c_int) {
    send_event(w, WindowEvent::Resize(width, height));
}

unsafe extern "C" fn handle_glfw_framebuffer_size(_w: GlfwWindow, width: c_int, height: c_int) {
    panic!("TODO: framebuffer_size callback {} {}", width, height);
}

unsafe extern "C" fn handle_glfw_window_close(w: GlfwWindow) {
    send_event(w, WindowEvent::Close);
}

unsafe fn send_event(win: GlfwWindow, event: WindowEvent) {
    let WinCtx { events_tx, notify } = &*(graffiti_glfw::glfwGetWindowUserPointer(win) as *mut _);

    events_tx.send(event);
    notify();
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
