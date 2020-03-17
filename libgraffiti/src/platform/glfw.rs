#![allow(non_snake_case, unused)] 

use crate::commons::{Au, Pos};
use crate::app::{WindowEvent};
use crate::platform::{NativeWindow, dylib_file};
use std::ptr;
use std::os::raw::{c_int, c_uint, c_double, c_void, c_char};

pub unsafe fn init() {
    silly!("loading glfw");
    load_glfw(c_str!(dylib_file("glfw", "3")));

    debug!("using glfw {:?}", std::ffi::CStr::from_ptr(glfwGetVersionString()));

    assert_eq!(glfwInit(), GLFW_TRUE, "init GLFW");

    #[cfg(target_os="macos")] {
        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 2);
        glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GLFW_TRUE);
        glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
    }
}

pub unsafe fn get_events(poll: bool) {
    if poll {
        glfwPollEvents()
    } else {
        // wait a bit otherwise (save battery)
        //
        // this number limits node.js responsivity
        // lower means sooner handling of I/O & timers
        // at the expense of some extra CPU overhead
        //
        // higher it is, more laggy it might feel
        // (http responses "taking too long", etc.)
        //
        // this number should be fine unless somebody is animating
        // with setTimeout or some other bad things
        //
        // ideally, we should just block with glfwWaitEvents()
        // but that would need somehow to send glfwPostEmptyEvent()
        // if anything in node.js is ready (not just I/O but also timers)
        // and it's not yet obvious to me how that could be done
        // so this is definitely good enough for now
        glfwWaitEventsTimeout(0.15);
    }
}

pub unsafe fn create_window(title: &str, width: i32, height: i32) -> NativeWindow {
    let w = glfwCreateWindow(width, height, c_str!(title), ptr::null_mut(), ptr::null_mut());

    if w.is_null() {
        let mut desc = std::ptr::null();

        let code = glfwGetError(&mut desc);

        panic!("create GLFW window, err {} {:?}", code, std::ffi::CStr::from_ptr(desc));
    }

    glfwMakeContextCurrent(w);

    glfwSetCursorPosCallback(w, handle_glfw_cursor_pos);
    glfwSetScrollCallback(w, handle_glfw_scroll);
    glfwSetMouseButtonCallback(w, handle_glfw_mouse_button);
    glfwSetCharCallback(w, handle_glfw_char);
    glfwSetKeyCallback(w, handle_glfw_key);
    glfwSetWindowSizeCallback(w, handle_glfw_window_size);
    glfwSetFramebufferSizeCallback(w, handle_glfw_framebuffer_size);
    glfwSetWindowCloseCallback(w, handle_glfw_window_close);

    // VSYNC=0 to disable
    let vsync = std::env::var("VSYNC").map(|s| s.parse().expect("vsync number")).unwrap_or(1);
    glfwSwapInterval(vsync);

    w as *mut c_void
}

pub unsafe fn make_current(native_window: NativeWindow) {
    glfwMakeContextCurrent(native_window as *mut GlfwWindow)
}

pub unsafe fn detach_current() {
    glfwMakeContextCurrent(ptr::null_mut());
}

pub unsafe fn swap_buffers(native_window: NativeWindow) {
    glfwSwapBuffers(native_window as *mut GlfwWindow)
}

unsafe extern "C" fn handle_glfw_cursor_pos(w: *mut GlfwWindow, x: c_double, y: c_double) {
    window_event!(w, w.mouse_move(Pos::new(x as Au, y as Au)))
}

unsafe extern "C" fn handle_glfw_scroll(_w: *mut GlfwWindow, _: c_double, _: c_double) {
    error!("TODO: handle_glfw_scroll");
}

unsafe extern "C" fn handle_glfw_mouse_button(w: *mut GlfwWindow, _button: c_int, action: c_int, _mods: c_int) {
    window_event!(w, match action {
        GLFW_PRESS => w.mouse_down(),
        GLFW_RELEASE => w.mouse_up(),
        _ => unreachable!("press/release expected"),
    })
}

unsafe extern "C" fn handle_glfw_key(w: *mut GlfwWindow, key: c_int, _scancode: c_int, action: c_int, _mods: c_int) {
    let key_code = get_key_code(key);

    window_event!(w, match action {
        // TODO: repeat works for some keys but for some it doesn't
        // not sure if it's specific for mac (special chars overlay)
        GLFW_RELEASE => w.key_up(key_code),
        _ => w.key_down(key_code),
    })
}

unsafe extern "C" fn handle_glfw_char(w: *mut GlfwWindow, char: c_uint) {
    window_event!(w, w.key_press(char as u16))
}

unsafe extern "C" fn handle_glfw_window_size(w: *mut GlfwWindow, width: c_int, height: c_int) {
    window_event!(w, w.resize((width as f32, height as f32)));
    glfwSwapBuffers(w);
}

unsafe extern "C" fn handle_glfw_framebuffer_size(_w: *mut GlfwWindow, width: c_int, height: c_int) {
    // TODO unpub
    crate::render::gl::set_curr_fb_size(width, height);
}

unsafe extern "C" fn handle_glfw_window_close(w: *mut GlfwWindow) {
    window_event!(w, w.close())
}

// from glfw to js `e.which`
// TODO: modifier (left/right for shift/ctrl/alt)
fn get_key_code(key: c_int) -> u16 {
    (match key {
        // some codes are the same
        32 | 48 ..= 57 | 65 ..= 90 => key,
        91 ..= 93 => key + 128,

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
        _ => 0
    }) as u16
}

// ffi

// struct without any field is not FFI-safe
pub enum GlfwWindow {}
pub enum GlfwMonitor {}

pub const GLFW_TRUE: c_int = 1;
pub const GLFW_FALSE: c_int = 0;
pub const GLFW_CONTEXT_VERSION_MAJOR: c_int = 0x0002_2002;
pub const GLFW_CONTEXT_VERSION_MINOR: c_int = 0x0002_2003;
pub const GLFW_OPENGL_FORWARD_COMPAT: c_int = 0x0002_2006;
pub const GLFW_OPENGL_PROFILE: c_int = 0x0002_2008;
pub const GLFW_OPENGL_CORE_PROFILE: c_int = 0x0003_2001;
pub const GLFW_RELEASE: c_int = 0;
pub const GLFW_PRESS: c_int = 1;

dylib! {
    #[load_glfw]
    extern "C" {
        fn glfwGetVersionString() -> *const c_char;
        fn glfwGetError(desc: *mut *const c_char) -> c_int;
        fn glfwInitHint(hint: c_int, value: c_int);
        fn glfwInit() -> c_int;

        fn glfwWindowHint(hint: c_int, value: c_int);
        fn glfwCreateWindow(width: c_int, height: c_int, title: *const c_char, monitor: *mut GlfwMonitor, share: *mut GlfwWindow) -> *mut GlfwWindow;
        fn glfwMakeContextCurrent(window: *mut GlfwWindow);
        fn glfwGetProcAddress(procname: *const c_char) -> *const c_void;
        fn glfwSwapInterval(interval: c_int);

        fn glfwSetCursorPosCallback(window: *mut GlfwWindow, cbfun: unsafe extern "C" fn(*mut GlfwWindow, c_double, c_double));
        fn glfwSetScrollCallback(window: *mut GlfwWindow, cbfun: unsafe extern "C" fn(*mut GlfwWindow, c_double, c_double));
        fn glfwSetMouseButtonCallback(window: *mut GlfwWindow, cbfun: unsafe extern "C" fn(*mut GlfwWindow, c_int, c_int, c_int));
        fn glfwSetKeyCallback(window: *mut GlfwWindow, cbfun: unsafe extern "C" fn(*mut GlfwWindow, c_int, c_int, c_int, c_int));
        fn glfwSetCharCallback(window: *mut GlfwWindow, cbfun: unsafe extern "C" fn(*mut GlfwWindow, c_uint));
        fn glfwSetWindowSizeCallback(window: *mut GlfwWindow, cbfun: unsafe extern "C" fn(*mut GlfwWindow, c_int, c_int));
        fn glfwSetFramebufferSizeCallback(window: *mut GlfwWindow, cbfun: unsafe extern "C" fn (*mut GlfwWindow, c_int, c_int));
        fn glfwSetWindowCloseCallback(window: *mut GlfwWindow, cbfun: unsafe extern "C" fn(*mut GlfwWindow));
        fn glfwPollEvents();
        fn glfwWaitEventsTimeout(timeout: f64);

        fn glfwSwapBuffers(window: *mut GlfwWindow);
    }
}
