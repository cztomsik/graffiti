use std::os::raw::{c_void, c_int, c_uint, c_char, c_double};

// link compiled glfw lib (static)
#[cfg(not(target_arch = "wasm32"))]
#[link(name = "glfw3", kind = "static")]
extern {}

// link glfw deps (dynamic)
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
#[cfg(target_os="windows")]
#[link(name = "opengl32")]
#[link(name = "gdi32")]
#[link(name = "user32")]
#[link(name = "shell32")]
extern {}

// TODO: consider newtype (and derive(Copy) & provide ::NULL)
pub type GlfwMonitor = *const c_void;
pub type GlfwWindow = *const c_void;

extern {
    pub fn glfwInit() -> c_int;

    pub fn glfwCreateWindow(width: c_int, height: c_int, title: *const c_char, monitor: GlfwMonitor, share: GlfwWindow) -> GlfwWindow;

    pub fn glfwWaitEventsTimeout(timeout: c_double);

    pub fn glfwPostEmptyEvent();

    pub fn glfwGetProcAddress(procname: *const c_char) -> *const c_void;

    pub fn glfwMakeContextCurrent(window: GlfwWindow);

    pub fn glfwSwapBuffers(window: GlfwWindow);
}
