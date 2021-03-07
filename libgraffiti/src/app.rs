use crate::{WebView, Window};
use core::cell::RefCell;
use graffiti_glfw::*;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::rc::{Rc, Weak};

pub struct App {
    weak: RefCell<Weak<Self>>,
}

impl App {
    pub unsafe fn init() -> Rc<Self> {
        assert_eq!(glfwInit(), GLFW_TRUE);

        glfwSetErrorCallback(handle_glfw_error);

        let res = Rc::new(Self {
            weak: RefCell::new(Weak::new()),
        });

        res.weak.replace(Rc::downgrade(&res));

        res
    }

    fn rc(&self) -> Rc<Self> {
        self.weak.borrow().upgrade().unwrap()
    }

    pub fn create_window(&self, title: &str, width: i32, height: i32) -> Window {
        Window::new(self.rc(), title, width, height)
    }

    pub fn create_webview(&self) -> WebView {
        WebView::new(self.rc())
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
