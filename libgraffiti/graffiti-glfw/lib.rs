use std::os::raw::{c_char, c_double, c_int, c_uint, c_void};

// link compiled glfw lib (static)
#[cfg(not(target_arch = "wasm32"))]
#[link(name = "glfw3", kind = "static")]
extern "C" {}

// link glfw deps (dynamic)
#[cfg(target_os = "linux")]
#[link(name = "X11")]
extern "C" {}
#[cfg(target_os = "macos")]
#[link(name = "CoreFoundation", kind = "framework")]
#[link(name = "Cocoa", kind = "framework")]
#[link(name = "IOKit", kind = "framework")]
#[link(name = "QuartzCore", kind = "framework")]
#[link(name = "OpenGL", kind = "framework")]
extern "C" {}
#[cfg(target_os = "windows")]
#[link(name = "opengl32")]
#[link(name = "gdi32")]
#[link(name = "user32")]
#[link(name = "shell32")]
extern "C" {}

pub const GLFW_TRUE: c_int = 1;
//pub const GLFW_FALSE: c_int = 0;
pub const GLFW_CONTEXT_VERSION_MAJOR: c_int = 0x0002_2002;
pub const GLFW_CONTEXT_VERSION_MINOR: c_int = 0x0002_2003;
pub const GLFW_OPENGL_FORWARD_COMPAT: c_int = 0x0002_2006;
pub const GLFW_OPENGL_PROFILE: c_int = 0x0002_2008;
pub const GLFW_OPENGL_CORE_PROFILE: c_int = 0x0003_2001;
pub const GLFW_RELEASE: c_int = 0;
pub const GLFW_PRESS: c_int = 1;

pub type GlfwMonitor = *mut c_void;
pub type GlfwWindow = *mut c_void;

extern "C" {
    pub fn glfwSetErrorCallback(cbfun: unsafe extern "C" fn(c_int, *const c_char)) -> *const c_void;
    pub fn glfwInit() -> c_int;

    pub fn glfwDefaultWindowHints();
    pub fn glfwWindowHint(hint: c_int, value: c_int);
    pub fn glfwCreateWindow(width: c_int, height: c_int, title: *const c_char, monitor: GlfwMonitor, share: GlfwWindow) -> GlfwWindow;

    pub fn glfwSetWindowUserPointer(window: GlfwWindow, ptr: *mut c_void);
    pub fn glfwGetWindowUserPointer(window: GlfwWindow) -> *mut c_void;

    pub fn glfwSetCursorPosCallback(window: GlfwWindow, cbfun: unsafe extern "C" fn(GlfwWindow, c_double, c_double));
    pub fn glfwSetScrollCallback(window: GlfwWindow, cbfun: unsafe extern "C" fn(GlfwWindow, c_double, c_double));
    pub fn glfwSetMouseButtonCallback(window: GlfwWindow, cbfun: unsafe extern "C" fn(GlfwWindow, c_int, c_int, c_int));
    pub fn glfwSetKeyCallback(window: GlfwWindow, cbfun: unsafe extern "C" fn(GlfwWindow, c_int, c_int, c_int, c_int));
    pub fn glfwSetCharCallback(window: GlfwWindow, cbfun: unsafe extern "C" fn(GlfwWindow, c_uint));
    pub fn glfwSetWindowSizeCallback(window: GlfwWindow, cbfun: unsafe extern "C" fn(GlfwWindow, c_int, c_int));
    pub fn glfwSetFramebufferSizeCallback(window: GlfwWindow, cbfun: unsafe extern "C" fn (GlfwWindow, c_int, c_int));
    pub fn glfwSetWindowCloseCallback(window: GlfwWindow, cbfun: unsafe extern "C" fn(GlfwWindow));

    pub fn glfwMakeContextCurrent(window: GlfwWindow);
    pub fn glfwSwapInterval(interval: c_int);
    pub fn glfwSwapBuffers(window: GlfwWindow);

    pub fn glfwGetProcAddress(procname: *const c_char) -> *const c_void;

    pub fn glfwPollEvents();
    pub fn glfwWaitEventsTimeout(timeout: c_double);
    pub fn glfwPostEmptyEvent();
}
