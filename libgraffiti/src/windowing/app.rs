// x should be !Send + !Sync
// x should outlive all the windows
//
// BTW: resize on macOS will block the main thread entirely, no matter if we are
//      waiting or just polling, and nothing will be rendered during that time
//      so we cannot have `app.tick()` rendering all windows and we also
//      cannot have all GL contexts owned by the main thread.
//      also, the whole idea of delayed app tasks does not make that much sense because again,
//      they will be blocked during resize.

use super::Window;
use graffiti_glfw::*;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct App(Weak<Self>);

impl App {
    pub fn init() -> Rc<Self> {
        // SAFETY: looking at the source code, it should be safe to call these
        unsafe {
            glfwSetErrorCallback(handle_glfw_error);
            assert_eq!(glfwInit(), GLFW_TRUE);
        }

        Rc::new_cyclic(|weak| Self(weak.clone()))
    }

    pub fn create_window(&self, title: &str, width: i32, height: i32) -> Window {
        Window::new(self.0.upgrade().unwrap(), title, width, height)
    }

    pub fn poll_events(&self) {
        unsafe { glfwPollEvents() }
    }

    pub fn wait_events(&self) {
        unsafe { glfwWaitEvents() }
    }

    pub fn wait_events_timeout(&self, timeout: f64) {
        unsafe { glfwWaitEventsTimeout(timeout) }
    }

    pub fn wake_up() {
        unsafe { glfwPostEmptyEvent() }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        unsafe { glfwTerminate() }
    }
}

unsafe extern "C" fn handle_glfw_error(code: c_int, desc: *const c_char) {
    eprintln!("GLFW error {} {:?}", code, CStr::from_ptr(desc));
}
