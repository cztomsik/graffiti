use std::os::raw::{c_char, c_double, c_float, c_int, c_uint, c_void};

// link compiled lib (static)
#[cfg(not(target_arch = "wasm32"))]
#[link(name = "glfw3", kind = "static")]
extern "C" {}

// link deps (dynamic)
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
pub const GLFW_FALSE: c_int = 0;

pub const GLFW_CLIENT_API: c_int = 0x00022001;
pub const GLFW_NO_API: c_int = 0x00000000;

pub const GLFW_FOCUSED: c_int = 0x00020001;
pub const GLFW_ICONIFIED: c_int = 0x00020002;
pub const GLFW_RESIZABLE: c_int = 0x00020003;
pub const GLFW_VISIBLE: c_int = 0x00020004;
pub const GLFW_DECORATED: c_int = 0x00020005;
pub const GLFW_AUTO_ICONIFY: c_int = 0x00020006;
pub const GLFW_FLOATING: c_int = 0x00020007;
pub const GLFW_MAXIMIZED: c_int = 0x00020008;
pub const GLFW_CENTER_CURSOR: c_int = 0x00020009;
pub const GLFW_TRANSPARENT_FRAMEBUFFER: c_int = 0x0002000A;
pub const GLFW_HOVERED: c_int = 0x0002000B;

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
    // init
    pub fn glfwGetVersionString() -> *const c_char;
    pub fn glfwSetErrorCallback(cbfun: unsafe extern "C" fn(c_int, *const c_char)) -> *const c_void;
    pub fn glfwInit() -> c_int;
    pub fn glfwTerminate();

    // create window
    pub fn glfwDefaultWindowHints();
    pub fn glfwWindowHint(hint: c_int, value: c_int);
    pub fn glfwCreateWindow(
        width: c_int,
        height: c_int,
        title: *const c_char,
        monitor: GlfwMonitor,
        share: GlfwWindow,
    ) -> GlfwWindow;
    pub fn glfwDestroyWindow(window: GlfwWindow);
    pub fn glfwSetWindowUserPointer(window: GlfwWindow, ptr: *mut c_void);
    pub fn glfwGetWindowUserPointer(window: GlfwWindow) -> *mut c_void;
    #[cfg(target_os = "macos")]
    pub fn glfwGetCocoaWindow(window: GlfwWindow) -> *mut c_void;

    // props
    pub fn glfwSetWindowTitle(window: GlfwWindow, title: *const c_char);
    pub fn glfwGetWindowSize(window: GlfwWindow, width: *mut c_int, height: *mut c_int);
    pub fn glfwSetWindowSize(window: GlfwWindow, width: c_int, height: c_int);
    pub fn glfwGetWindowPos(window: GlfwWindow, xpos: *mut c_int, ypos: *mut c_int);
    pub fn glfwSetWindowPos(window: GlfwWindow, xpos: c_int, ypos: c_int);
    pub fn glfwGetWindowOpacity(window: GlfwWindow) -> c_float;
    pub fn glfwSetWindowOpacity(window: GlfwWindow, opacity: c_float);
    pub fn glfwGetWindowAttrib(window: GlfwWindow, attrib: c_int) -> c_int;
    pub fn glfwSetWindowAttrib(window: GlfwWindow, attrib: c_int, value: c_int);

    // actions
    pub fn glfwShowWindow(window: GlfwWindow);
    pub fn glfwHideWindow(window: GlfwWindow);
    pub fn glfwFocusWindow(window: GlfwWindow);
    pub fn glfwIconifyWindow(window: GlfwWindow);
    pub fn glfwMaximizeWindow(window: GlfwWindow);
    pub fn glfwRestoreWindow(window: GlfwWindow);
    pub fn glfwRequestWindowAttention(window: GlfwWindow);

    // event listeners
    pub fn glfwSetCursorPosCallback(window: GlfwWindow, cbfun: unsafe extern "C" fn(GlfwWindow, c_double, c_double));
    pub fn glfwSetScrollCallback(window: GlfwWindow, cbfun: unsafe extern "C" fn(GlfwWindow, c_double, c_double));
    pub fn glfwSetMouseButtonCallback(window: GlfwWindow, cbfun: unsafe extern "C" fn(GlfwWindow, c_int, c_int, c_int));
    pub fn glfwSetKeyCallback(window: GlfwWindow, cbfun: unsafe extern "C" fn(GlfwWindow, c_int, c_int, c_int, c_int));
    pub fn glfwSetCharCallback(window: GlfwWindow, cbfun: unsafe extern "C" fn(GlfwWindow, c_uint));
    pub fn glfwSetWindowSizeCallback(window: GlfwWindow, cbfun: unsafe extern "C" fn(GlfwWindow, c_int, c_int));
    pub fn glfwSetFramebufferSizeCallback(window: GlfwWindow, cbfun: unsafe extern "C" fn(GlfwWindow, c_int, c_int));
    pub fn glfwSetWindowCloseCallback(window: GlfwWindow, cbfun: unsafe extern "C" fn(GlfwWindow));

    // event loop
    pub fn glfwWindowShouldClose(window: GlfwWindow) -> c_int;
    pub fn glfwSetWindowShouldClose(window: GlfwWindow, value: c_int);
    pub fn glfwPollEvents();
    pub fn glfwWaitEvents();
    pub fn glfwWaitEventsTimeout(timeout: c_double);
    pub fn glfwPostEmptyEvent();

    // GL
    pub fn glfwGetProcAddress(procname: *const c_char) -> *const c_void;
    pub fn glfwGetCurrentContext() -> GlfwWindow;
    pub fn glfwMakeContextCurrent(window: GlfwWindow);
    pub fn glfwGetFramebufferSize(window: GlfwWindow, width: *mut c_int, height: *mut c_int);
    pub fn glfwGetWindowContentScale(window: GlfwWindow, xscale: *mut c_float, yscale: *mut c_float);
    pub fn glfwSwapInterval(interval: c_int);
    pub fn glfwSwapBuffers(window: GlfwWindow);

    // misc
    pub fn glfwGetClipboardString(window: GlfwWindow) -> *const c_char;
    pub fn glfwSetClipboardString(window: GlfwWindow, string: *const c_char);
}
