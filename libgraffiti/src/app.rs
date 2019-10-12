use crate::commons::{Au, Pos};
use crate::window::{Window, Event, UpdateSceneMsg};
use std::collections::BTreeMap;


// - create/destroy windows
// - access them with id
// - get pending events (with surface targets) of all windows
pub struct TheApp {
    windows: BTreeMap<*mut GlfwWindow, (Window, WindowId)>,

    next_window_id: WindowId,
}

pub type WindowId = usize;

impl TheApp {
    pub unsafe fn init() -> Self {
        assert_eq!(glfwInit(), GLFW_TRUE, "init GLFW");

        #[cfg(target_os="macos")] {
            glfwInitHint(GLFW_COCOA_CHDIR_RESOURCES, GLFW_FALSE);

            glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
            glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 2);
            glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GLFW_TRUE);
            glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
        }

        TheApp {
            windows: BTreeMap::new(),
            next_window_id: 1,
        }
    }
}

impl TheApp {
    pub fn get_events(&mut self, poll: bool) -> Vec<Event> {
        // TODO: share the vec, clear it only
        // maybe it can be part of App state?
        let mut events = Vec::new();

        unsafe {
            WINDOWS_PTR = &mut self.windows;
            PENDING_EVENTS_PTR = &mut events;

            if poll {
                glfwPollEvents()
            } else {
                // wait a bit otherwise (save battery)
                glfwWaitEventsTimeout(0.1);
            }

            PENDING_EVENTS_PTR = ptr::null_mut();
        }

        events
    }

    pub fn create_window(&mut self) -> WindowId {
        let (width, height) = (1024, 768);
        let id = self.next_window_id;

        let glfw_window = unsafe {
            let w = glfwCreateWindow(width, height, c_str!("graffiti"), ptr::null_mut(), ptr::null_mut());
            assert!(!w.is_null(), "create GLFW window");

            glfwMakeContextCurrent(w);
            gl::load_with(|addr| glfwGetProcAddress(c_str!(addr)));

            glfwSetCursorPosCallback(w, handle_glfw_cursor_pos);
            glfwSetScrollCallback(w, handle_glfw_scroll);
            glfwSetMouseButtonCallback(w, handle_glfw_mouse_button);
            glfwSetCharCallback(w, handle_glfw_char);
            glfwSetKeyCallback(w, handle_glfw_key);
            glfwSetWindowSizeCallback(w, handle_glfw_window_size);
            glfwSetFramebufferSizeCallback(w, handle_glfw_framebuffer_size);
            glfwSetWindowCloseCallback(w, handle_glfw_window_close);

            // vsync off (for now)
            glfwSwapInterval(0);

            w
        };

        let window = Window::new(width, height);

        self.windows.insert(glfw_window, (window, id));

        self.next_window_id = self.next_window_id + 1;

        id
    }

    pub fn update_window_scene(&mut self, id: WindowId, msg: &UpdateSceneMsg) {
        // can be values_mut once we have glfw_window in window
        for (glfw_window, (w, w_id)) in self.windows.iter_mut() {
            if *w_id == id {
                w.update_scene(msg);
                unsafe { glfwSwapBuffers(*glfw_window) };
                return;
            }
        }

        error!("got msg for nonexisting window {:?} {:?}", id, msg);
    }

    pub fn destroy_window(&mut self, _id: WindowId) {
        // TODO
        //self.windows.remove(&id);
    }
}

static mut WINDOWS_PTR: *mut BTreeMap<*mut GlfwWindow, (Window, WindowId)> = ptr::null_mut();
static mut PENDING_EVENTS_PTR: *mut Vec<Event> = ptr::null_mut();


// TODO: extract platform & move this to platform/glfw.rs

use std::ptr;
use libc::{c_void, c_int, c_uint, c_char, c_double};

// needed otherwise link wont work
// (emscripten does not need it)
#[cfg(not(target_arch = "wasm32"))]
#[allow(unused_imports)]
use glfw_sys;

#[link(name = "glfw3", kind = "static")]
extern {}

#[cfg(target_os="linux")]
#[link(name = "X11")]
extern {}

#[cfg(target_os="macos")]
#[link(name = "CoreFoundation", kind = "framework")]
#[link(name = "Cocoa", kind = "framework")]
#[link(name = "IOKit", kind = "framework")]
#[link(name = "QuartzCore", kind = "framework")]
#[link(name = "OpenGL", kind = "framework")]
extern {}

// struct without any field is not FFI-safe
pub enum GlfwWindow {}
pub enum GlfwMonitor {}

const GLFW_TRUE: c_int = 1;
const GLFW_FALSE: c_int = 0;
const GLFW_COCOA_CHDIR_RESOURCES: c_int = 0x00051001;
const GLFW_CONTEXT_VERSION_MAJOR: c_int = 0x00022002;
const GLFW_CONTEXT_VERSION_MINOR: c_int = 0x00022003;
const GLFW_OPENGL_FORWARD_COMPAT: c_int = 0x00022006;
const GLFW_OPENGL_PROFILE: c_int = 0x00022008;
const GLFW_OPENGL_CORE_PROFILE: c_int = 0x00032001;
const GLFW_RELEASE: c_int = 0;
const GLFW_PRESS: c_int = 1;
const GLFW_REPEAT: c_int = 2;


extern "C" {
    pub fn glfwInitHint(hint: c_int, value: c_int);
    pub fn glfwInit() -> c_int;

    pub fn glfwWindowHint(hint: c_int, value: c_int);
    pub fn glfwCreateWindow(width: c_int, height: c_int, title: *const c_char, monitor: *mut GlfwMonitor, share: *mut GlfwWindow) -> *mut GlfwWindow;
    pub fn glfwGetCurrentContext() -> *mut GlfwWindow;
    pub fn glfwMakeContextCurrent(window: *mut GlfwWindow);
    pub fn glfwGetProcAddress(procname: *const c_char) -> *const c_void;
    pub fn glfwSwapInterval(interval: c_int);

    pub fn glfwSetCursorPosCallback(window: *mut GlfwWindow, cbfun: unsafe extern "C" fn(*mut GlfwWindow, c_double, c_double));
    pub fn glfwSetScrollCallback(window: *mut GlfwWindow, cbfun: unsafe extern "C" fn(*mut GlfwWindow, c_double, c_double));
    pub fn glfwSetMouseButtonCallback(window: *mut GlfwWindow, cbfun: unsafe extern "C" fn(*mut GlfwWindow, c_int, c_int, c_int));
    pub fn glfwSetKeyCallback(window: *mut GlfwWindow, cbfun: unsafe extern "C" fn(*mut GlfwWindow, c_int, c_int, c_int, c_int));
    pub fn glfwSetCharCallback(window: *mut GlfwWindow, cbfun: unsafe extern "C" fn(*mut GlfwWindow, c_uint));
    pub fn glfwSetWindowSizeCallback(window: *mut GlfwWindow, cbfun: unsafe extern "C" fn(*mut GlfwWindow, c_int, c_int));
    pub fn glfwSetFramebufferSizeCallback(window: *mut GlfwWindow, cbfun: unsafe extern "C" fn (*mut GlfwWindow, c_int, c_int));
    pub fn glfwSetWindowCloseCallback(window: *mut GlfwWindow, cbfun: unsafe extern "C" fn(*mut GlfwWindow));
    pub fn glfwPollEvents();
    pub fn glfwWaitEventsTimeout(timeout: f32);

    pub fn glfwSwapBuffers(window: *mut GlfwWindow);
}

// function is not enough because the closure captures the args
macro_rules! window_event {
    ($w:ident, $body:expr) => {{
        let ($w, _id) = (*WINDOWS_PTR).get_mut(&$w).expect("missing window");
        let event = $body;

        (*PENDING_EVENTS_PTR).push(event);
    }}
}

unsafe extern "C" fn handle_glfw_cursor_pos(w: *mut GlfwWindow, x: c_double, y: c_double) {
    window_event!(w, w.mouse_move(Pos::new(x as Au, y as Au)))
}

unsafe extern "C" fn handle_glfw_scroll(_w: *mut GlfwWindow, _: c_double, _: c_double) {
    debug!("TODO: handle_glfw_scroll");
}

unsafe extern "C" fn handle_glfw_mouse_button(w: *mut GlfwWindow, _button: c_int, action: c_int, _mods: c_int) {
    window_event!(w, match action {
        GLFW_PRESS => w.mouse_down(),
        GLFW_RELEASE => w.mouse_up(),
        _ => unreachable!("press/release expected"),
    })
}

unsafe extern "C" fn handle_glfw_key(w: *mut GlfwWindow, _key: c_int, scancode: c_int, action: c_int, _mods: c_int) {
    window_event!(w, match action {
        // TODO: repeat works for some keys but for some it doesn't
        // not sure if it's specific for mac (special chars overlay)
        GLFW_RELEASE => w.key_up(scancode as u16),
        _ => w.key_down(scancode as u16),
    })
}
unsafe extern "C" fn handle_glfw_char(w: *mut GlfwWindow, char: c_uint) {
    window_event!(w, w.key_press(char as u16))
}

unsafe extern "C" fn handle_glfw_window_size(w: *mut GlfwWindow, width: c_int, height: c_int) {
    window_event!(w, w.resize(width, height));
    glfwSwapBuffers(w);
}

unsafe extern "C" fn handle_glfw_framebuffer_size(_w: *mut GlfwWindow, width: c_int, height: c_int) {
    gl::Viewport(0, 0, width, height);
}

unsafe extern "C" fn handle_glfw_window_close(w: *mut GlfwWindow) {
    window_event!(w, w.close())
}
