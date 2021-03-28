use graffiti_glfw::*;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::rc::Rc;

pub struct App {
    // !Send, !Sync
    marker: *mut (),
}

impl App {
    pub unsafe fn init() -> Rc<Self> {
        assert_eq!(glfwInit(), GLFW_TRUE);

        glfwSetErrorCallback(handle_glfw_error);

        Rc::new(Self {
            marker: std::ptr::null_mut(),
        })
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
