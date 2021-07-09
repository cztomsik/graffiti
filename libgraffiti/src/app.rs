#![allow(clippy::missing_safety_doc)]

use graffiti_glfw::*;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::{c_char, c_int};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crossbeam_channel::{unbounded as channel, Receiver, Sender};
use once_cell::sync::Lazy;

thread_local! {
    static APP: RefCell<Weak<App>> = Default::default();
}

pub type AppTask = Box<dyn FnOnce() + 'static + Send>;

// TODO: consider removing crossbeam but it's also used by Window
static TASK_CHANNEL: Lazy<(Sender<AppTask>, Receiver<AppTask>)> = Lazy::new(channel);

pub struct App {
    // !Send, !Sync
    _marker: PhantomData<*mut ()>,
}

impl App {
    pub unsafe fn init() -> Rc<Self> {
        // TODO: check main thread
        assert_eq!(glfwInit(), GLFW_TRUE);
        glfwSetErrorCallback(handle_glfw_error);

        let rc = Rc::new(Self { _marker: PhantomData });

        APP.with(|weak| weak.replace(Rc::downgrade(&rc)));

        rc
    }

    pub fn current() -> Option<Rc<Self>> {
        APP.with(|weak| Weak::upgrade(&weak.borrow()))
    }

    pub fn tick(&self) {
        self.run_tasks();
        self.wait_events_timeout(0.1);
    }
    
    pub fn run_tasks(&self) {
        TASK_CHANNEL.1.try_iter().for_each(|t| t());
    }

    pub fn push_task(&self, task: impl FnOnce() + 'static + Send) {
        TASK_CHANNEL.0.send(Box::new(task));
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

        APP.with(|weak| weak.take());
    }
}

unsafe extern "C" fn handle_glfw_error(code: c_int, desc: *const c_char) {
    eprintln!("GLFW error {} {:?}", code, CStr::from_ptr(desc));
}
