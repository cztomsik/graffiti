use std::os::raw::{c_void, c_int, c_uint, c_char, c_double};

// (emscripten does not need it)
//
// note that for dylib it's not enough to just change the kind
// it's also necessary to not emit libglfw3.a in build.rs (not 100% sure why)
//
// but glfw is just 120 KB so I guess it's fine to link static for now and
// it should be carefully evaluated in what exactly the dylib could be better
//
// I can see value in SDL2 (android, iOS) and that one is much bigger (1MB)
// so using system-wide version might be a good idea there
// but we need very little of its functionality so glfw feels like better fit
// and mobile is not on the roadmap yet anyway (but it's something worth thinking)
#[cfg(not(target_arch = "wasm32"))]
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

pub const GLFW_TRUE: c_int = 1;
pub const GLFW_FALSE: c_int = 0;
pub const GLFW_COCOA_CHDIR_RESOURCES: c_int = 0x00051001;
pub const GLFW_CONTEXT_VERSION_MAJOR: c_int = 0x00022002;
pub const GLFW_CONTEXT_VERSION_MINOR: c_int = 0x00022003;
pub const GLFW_OPENGL_FORWARD_COMPAT: c_int = 0x00022006;
pub const GLFW_OPENGL_PROFILE: c_int = 0x00022008;
pub const GLFW_OPENGL_CORE_PROFILE: c_int = 0x00032001;
pub const GLFW_RELEASE: c_int = 0;
pub const GLFW_PRESS: c_int = 1;
pub const GLFW_REPEAT: c_int = 2;

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
    pub fn glfwWaitEvents();
    pub fn glfwWaitEventsTimeout(timeout: f64);

    pub fn glfwSwapBuffers(window: *mut GlfwWindow);
}
