use crossbeam_channel::{unbounded as channel, Receiver, Sender};
use graffiti_glfw::*;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::rc::{Rc, Weak};

pub type AppTask = Box<dyn FnOnce() + 'static + Send>;

static TASKS_CHAN: Lazy<(Sender<AppTask>, Receiver<AppTask>)> = Lazy::new(|| channel());

thread_local! {
    static APP: RefCell<Weak<App>> = Default::default();
}

pub struct App;

impl App {
    pub unsafe fn init() -> Rc<Self> {
        // TODO: can we check main thread before glfw fails?

        assert!(Self::current().is_none(), "already initialized");

        assert_eq!(glfwInit(), GLFW_TRUE);
        glfwSetErrorCallback(handle_glfw_error);

        let app = Rc::new(Self);

        APP.with(|dest| *dest.borrow_mut() = Rc::downgrade(&app));

        app
    }

    pub fn current() -> Option<Rc<Self>> {
        APP.with(|app| Weak::upgrade(&app.borrow()))
    }

    pub fn push_task(task: impl FnOnce() + 'static + Send) {
        TASKS_CHAN.0.send(Box::new(task)).unwrap();
    }

    pub fn await_task<T: Send + 'static>(task: impl FnOnce() -> T + 'static + Send) -> T {
        if Self::current().is_some() {
            task()
        } else {
            let (tx, rx) = channel();
            Self::push_task(move || tx.send(task()).unwrap());
            rx.recv().unwrap()
        }
    }

    pub fn tick(&self) {
        TASKS_CHAN.1.try_iter().for_each(|t| t());
        self.wait_events_timeout(0.1);
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

        APP.with(|app| *app.borrow_mut() = Weak::default());
    }
}

unsafe extern "C" fn handle_glfw_error(code: c_int, desc: *const c_char) {
    eprintln!("GLFW error {} {:?}", code, CStr::from_ptr(desc));
}
