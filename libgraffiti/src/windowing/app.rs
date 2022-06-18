// x should be !Send + !Sync
// x should outlive all the windows
// x should allow pushing main-thread tasks

use super::Window;
use crossbeam_channel::{unbounded as channel, Receiver, Sender};
use graffiti_glfw::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::ffi::CStr;
use std::fmt;
use std::marker::PhantomData;
use std::os::raw::{c_char, c_int};

pub type WindowId = usize;

pub type AppTask = Box<dyn FnOnce(&mut App) + 'static + Send>;

static TASKS_CHAN: Lazy<(Sender<AppTask>, Receiver<AppTask>)> = Lazy::new(channel);

pub struct App {
    windows: HashMap<WindowId, Window>,
    marker: PhantomData<*const ()>,
}

impl App {
    pub fn init() -> Self {
        // SAFETY: looking at the source code, it should be safe to call these
        unsafe {
            assert_eq!(glfwInit(), GLFW_TRUE);
            glfwSetErrorCallback(handle_glfw_error);
        }

        Self {
            windows: HashMap::new(),
            marker: PhantomData,
        }
    }

    pub fn create_window(&mut self, title: &str, width: i32, height: i32) -> WindowId {
        let win = Window::new(title, width, height);
        let id = win.id();

        self.windows.insert(id, win);

        id
    }

    pub fn windows(&self) -> impl Iterator<Item = &Window> {
        self.windows.values()
    }

    pub fn windows_mut(&mut self) -> impl Iterator<Item = &mut Window> {
        self.windows.values_mut()
    }

    pub fn window(&self, id: WindowId) -> &Window {
        &self.windows[&id]
    }

    pub fn window_mut(&mut self, id: WindowId) -> &mut Window {
        self.windows.get_mut(&id).unwrap()
    }

    pub fn drop_window(&mut self, id: WindowId) {
        self.windows.remove(&id);
    }

    pub fn push_task(task: impl FnOnce(&mut Self) + 'static + Send) {
        TASKS_CHAN.0.send(Box::new(task)).unwrap();
    }

    pub fn tick(&mut self) {
        for task in TASKS_CHAN.1.try_iter() {
            task(self);
        }

        for win in self.windows_mut() {
            win.render();
        }

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
    }
}

impl fmt::Debug for App {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("App").finish()
    }
}

unsafe extern "C" fn handle_glfw_error(code: c_int, desc: *const c_char) {
    eprintln!("GLFW error {} {:?}", code, CStr::from_ptr(desc));
}
