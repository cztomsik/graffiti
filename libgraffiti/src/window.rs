#![allow(non_snake_case, unused)]

use std::os::raw::{c_char, c_double, c_int, c_uint, c_void};

// mozna window manager, nevim, ale zatim me nic lepsiho nenapadlo

pub unsafe fn init() {
    silly!("loading glfw");
    load_glfw(c_str!(crate::util::dylib::dylib_file("glfw", "3")));

    debug!("using glfw {:?}", std::ffi::CStr::from_ptr(glfwGetVersionString()));

    //silly!("setting err callback");
    //glfwSetErrorCallback(handle_glfw_error);

    assert_eq!(glfwInit(), GLFW_TRUE, "init GLFW");
}

pub unsafe fn create_window(title: &str, width: i32, height: i32) -> Window {
    /*
    glfwDefaultWindowHints();

    #[cfg(target_os="macos")] {
        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 2);
        glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GLFW_TRUE);
        glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
    }
    */

    let native_window = glfwCreateWindow(width, height, c_str!(title), std::ptr::null_mut(), std::ptr::null_mut());

    assert_ne!(native_window, std::ptr::null_mut(), "create GLFW window");

    glfwMakeContextCurrent(native_window);

    // TODO: init listeners

    // TODO: init viewport

    // detach so it can be attached in other thread
    glfwMakeContextCurrent(std::ptr::null_mut());

    Window { native_window }
}

pub unsafe fn wait_events(timeout_ms: Option<i32>) {
    match timeout_ms {
        None => glfwWaitEvents(),
        Some(0) => glfwPollEvents(),
        Some(t) => glfwWaitEventsTimeout(t as f64 / 1000.)
    }
}

pub unsafe fn wakeup() {
    glfwPostEmptyEvent()
}

pub struct Window {
    native_window: *mut GlfwWindow
}

impl Window {
    pub unsafe fn render(&mut self) {
        glfwMakeContextCurrent(self.native_window);

        //viewport.render();

        // note that if it gets called too fast it can
        // block until the next frame
        //
        glfwSwapBuffers(self.native_window);
    }
}





enum GlfwWindow {}
enum GlfwMonitor {}

const GLFW_TRUE: c_int = 1;
const GLFW_FALSE: c_int = 0;
const GLFW_CONTEXT_VERSION_MAJOR: c_int = 0x0002_2002;
const GLFW_CONTEXT_VERSION_MINOR: c_int = 0x0002_2003;
const GLFW_OPENGL_FORWARD_COMPAT: c_int = 0x0002_2006;
const GLFW_OPENGL_PROFILE: c_int = 0x0002_2008;
const GLFW_OPENGL_CORE_PROFILE: c_int = 0x0003_2001;
const GLFW_RELEASE: c_int = 0;
const GLFW_PRESS: c_int = 1;

dylib! {
    #[load_glfw]
    extern "C" {
        fn glfwGetVersionString() -> *const c_char;
        fn glfwSetErrorCallback(cbfun: unsafe extern "C" fn(c_int, *const c_char)) -> *const c_void;
        fn glfwInit() -> c_int;

        fn glfwWindowHint(hint: c_int, value: c_int);
        fn glfwCreateWindow(width: c_int, height: c_int, title: *const c_char, monitor: *mut GlfwMonitor, share: *mut GlfwWindow) -> *mut GlfwWindow;
        fn glfwMakeContextCurrent(window: *mut GlfwWindow);
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
        fn glfwWaitEvents();
        fn glfwWaitEventsTimeout(timeout: f64);
        fn glfwPostEmptyEvent();

        fn glfwSwapBuffers(window: *mut GlfwWindow);
    }
}
