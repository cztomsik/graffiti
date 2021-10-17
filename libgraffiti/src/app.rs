#![allow(clippy::missing_safety_doc)]

use crossbeam_channel::{unbounded as channel, Receiver, Sender};
use graffiti_glfw::*;
use once_cell::sync::Lazy;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::sync::{Arc, Mutex, Weak};
use std::thread::ThreadId;

pub type AppTask = Box<dyn FnOnce() + 'static + Send>;

static APP: Lazy<Mutex<Weak<App>>> = Lazy::new(Default::default);

pub struct App {
    main_thread_id: ThreadId,
    task_tx: Sender<AppTask>,
    task_rx: Receiver<AppTask>,
}

impl App {
    pub unsafe fn init() -> Arc<Self> {
        // TODO: check main thread

        if Self::current().is_some() {
            panic!("already initialized");
        }

        assert_eq!(glfwInit(), GLFW_TRUE);
        glfwSetErrorCallback(handle_glfw_error);

        let main_thread_id = std::thread::current().id();
        let (task_tx, task_rx) = channel();

        let app = Arc::new(Self {
            main_thread_id,
            task_tx,
            task_rx,
        });

        *APP.lock().unwrap() = Arc::downgrade(&app);

        app
    }

    pub fn current() -> Option<Arc<Self>> {
        Weak::upgrade(&APP.lock().unwrap())
    }

    pub fn is_main_thread(&self) -> bool {
        std::thread::current().id() == self.main_thread_id
    }

    pub fn push_task(&self, task: impl FnOnce() + 'static + Send) {
        self.task_tx.send(Box::new(task)).expect("app down");
    }

    pub fn await_task<T: Send + 'static>(&self, task: impl FnOnce() -> T + 'static + Send) -> T {
        if self.is_main_thread() {
            task()
        } else {
            let (tx, rx) = channel();
            self.push_task(move || tx.send(task()).unwrap());
            rx.recv().unwrap()
        }
    }

    pub fn run_tasks(&self) {
        assert!(self.is_main_thread());
        self.task_rx.try_iter().for_each(|t| t());
    }

    pub fn tick(&self) {
        assert!(self.is_main_thread());
        self.run_tasks();
        self.wait_events_timeout(0.1);
    }

    pub fn poll_events(&self) {
        assert!(self.is_main_thread());
        unsafe { glfwPollEvents() }
    }

    pub fn wait_events(&self) {
        assert!(self.is_main_thread());
        unsafe { glfwWaitEvents() }
    }

    pub fn wait_events_timeout(&self, timeout: f64) {
        assert!(self.is_main_thread());
        unsafe { glfwWaitEventsTimeout(timeout) }
    }

    pub fn wake_up(&self) {
        unsafe { glfwPostEmptyEvent() }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        // TODO: maybe this is a sign that there should be also some !Send AppRef
        //       or maybe EventLoop should be separate and (and referencing App)
        assert!(self.is_main_thread());

        unsafe { glfwTerminate() }

        *APP.lock().unwrap() = Default::default();
    }
}

// wrapper for !Send resources
#[derive(Clone)]
pub(crate) struct AppOwned<T: Clone>(pub(crate) T);

unsafe impl<T: Clone> Send for AppOwned<T> {}
unsafe impl<T: Clone> Sync for AppOwned<T> {}

impl<T: Clone + 'static> AppOwned<T> {
    // TODO: find a better name? 
    // execute task on main thread with clone of T
    // will either block or panic so it should be safe
    pub(crate) fn with<R: 'static + Send>(&self, fun: impl FnOnce(T) -> R + Send + 'static) -> R {
        let handle = self.clone();
        App::current().unwrap().await_task(move || fun(handle.0))
    }
}

unsafe extern "C" fn handle_glfw_error(code: c_int, desc: *const c_char) {
    eprintln!("GLFW error {} {:?}", code, CStr::from_ptr(desc));
}
