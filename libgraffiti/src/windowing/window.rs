// we can't use glfw-rs because we need callbacks (for real-time resize)
// whereas they just sink everything into channel and handle it when it's too late
//
// also, we might replace glfw with some rust-native crate in future
// and we might also consider adding support for mobile and glfw cannot
// do that currently

use super::Event;
use crate::{Renderer, Viewport};
use graffiti_glfw::*;
use std::cell::RefCell;
use std::ffi::CString;
use std::fmt;
use std::os::raw::{c_double, c_int, c_uint, c_void};
use std::ptr::null_mut;
use std::sync::{Arc, RwLock};

pub struct Window {
    renderer: Renderer,
    glfw_window: GlfwWindow,
    content: Option<Arc<RwLock<Viewport>>>,

    // glfw does not provide getter
    title: RefCell<String>,
}

impl Window {
    pub(super) fn new(title: &str, width: i32, height: i32) -> Self {
        let c_title = CString::new(title).unwrap();

        let (glfw_window, renderer) = unsafe {
            glfwDefaultWindowHints();

            #[cfg(target_os = "macos")]
            {
                glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
                glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 2);
                glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GLFW_TRUE);
                glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
            }

            let glfw_window = glfwCreateWindow(width, height, c_title.as_ptr(), null_mut(), null_mut());
            assert_ne!(glfw_window, null_mut(), "create GLFW window");

            // TODO: set_handler() ?
            // glfwSetWindowUserPointer(glfw_window, &*content as *const _ as _);
            glfwSetCursorPosCallback(glfw_window, handle_glfw_cursor_pos);
            glfwSetScrollCallback(glfw_window, handle_glfw_scroll);
            glfwSetMouseButtonCallback(glfw_window, handle_glfw_mouse_button);
            glfwSetCharCallback(glfw_window, handle_glfw_char);
            glfwSetKeyCallback(glfw_window, handle_glfw_key);
            glfwSetWindowSizeCallback(glfw_window, handle_glfw_window_size);
            glfwSetFramebufferSizeCallback(glfw_window, handle_glfw_framebuffer_size);
            glfwSetWindowCloseCallback(glfw_window, handle_glfw_window_close);

            glfwMakeContextCurrent(glfw_window);
            let mut fb_size = (0, 0);
            glfwGetFramebufferSize(glfw_window, &mut fb_size.0, &mut fb_size.1);
            let mut scale = (0., 0.);
            glfwGetWindowContentScale(glfw_window, &mut scale.0, &mut scale.1);
            let renderer = Renderer::new(fb_size, (scale.0 as _, scale.1 as _), |sym| {
                let sym = CString::new(sym).unwrap();
                glfwGetProcAddress(sym.as_ptr())
            });

            // detach
            glfwMakeContextCurrent(std::ptr::null_mut());

            (glfw_window, renderer)
        };

        Self {
            renderer,
            content: None,

            title: RefCell::new(title.to_owned()),
            glfw_window,
        }
    }

    pub(super) fn id(&self) -> usize {
        self.glfw_window as _
    }

    pub fn render(&mut self) {
        unsafe { glfwMakeContextCurrent(self.glfw_window) };

        if let Some(content) = &self.content {
            self.renderer.render(&mut *content.write().unwrap());
        }

        unsafe {
            glfwSwapBuffers(self.glfw_window);
            glfwMakeContextCurrent(std::ptr::null_mut())
        };
    }

    pub fn content(&self) -> Option<&Arc<RwLock<Viewport>>> {
        self.content.as_ref()
    }

    pub fn set_content(&mut self, content: Option<Arc<RwLock<Viewport>>>) {
        self.content = content;
    }

    pub fn title(&self) -> String {
        self.title.borrow().clone()
    }

    pub fn set_title(&self, title: &str) {
        *self.title.borrow_mut() = title.to_owned();

        let title = CString::new(title).unwrap();
        unsafe { glfwSetWindowTitle(self.glfw_window, title.as_ptr()) };
    }

    pub fn resizable(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.glfw_window, GLFW_RESIZABLE) == GLFW_TRUE }
    }

    pub fn set_resizable(&self, resizable: bool) {
        unsafe { glfwSetWindowAttrib(self.glfw_window, GLFW_RESIZABLE, resizable as _) };
    }

    pub fn size(&self) -> (i32, i32) {
        let mut size = (0, 0);
        unsafe { glfwGetWindowSize(self.glfw_window, &mut size.0, &mut size.1) }
        size
    }

    pub fn set_size(&self, (width, height): (i32, i32)) {
        unsafe { glfwSetWindowSize(self.glfw_window, width as _, height as _) };
    }

    pub fn framebuffer_size(&self) -> (i32, i32) {
        let mut size = (0, 0);
        unsafe { glfwGetFramebufferSize(self.glfw_window, &mut size.0, &mut size.1) }
        size
    }

    pub fn content_scale(&self) -> (f32, f32) {
        let mut scale = (0., 0.);
        unsafe { glfwGetWindowContentScale(self.glfw_window, &mut scale.0, &mut scale.1) }
        scale
    }

    pub fn transparent(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.glfw_window, GLFW_TRANSPARENT_FRAMEBUFFER) == GLFW_TRUE }
    }

    pub fn opacity(&self) -> f32 {
        unsafe { glfwGetWindowOpacity(self.glfw_window) }
    }

    pub fn set_opacity(&self, opacity: f32) {
        unsafe { glfwSetWindowOpacity(self.glfw_window, opacity) };
    }

    pub fn visible(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.glfw_window, GLFW_VISIBLE) == GLFW_TRUE }
    }

    pub fn show(&self) {
        unsafe { glfwShowWindow(self.glfw_window) };
    }

    pub fn hide(&self) {
        unsafe { glfwHideWindow(self.glfw_window) };
    }

    pub fn focused(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.glfw_window, GLFW_FOCUSED) == GLFW_TRUE }
    }

    pub fn focus(&self) {
        unsafe { glfwFocusWindow(self.glfw_window) };
    }

    pub fn minimized(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.glfw_window, GLFW_ICONIFIED) == GLFW_TRUE }
    }

    pub fn minimize(&self) {
        unsafe { glfwIconifyWindow(self.glfw_window) };
    }

    pub fn maximized(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.glfw_window, GLFW_MAXIMIZED) == GLFW_TRUE }
    }

    pub fn maximize(&self) {
        unsafe { glfwMaximizeWindow(self.glfw_window) };
    }

    pub fn restore(&self) {
        unsafe { glfwRestoreWindow(self.glfw_window) };
    }

    pub fn request_attention(&self) {
        unsafe { glfwRequestWindowAttention(self.glfw_window) };
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            let ptr = glfwGetWindowUserPointer(self.glfw_window);
            glfwSetWindowUserPointer(self.glfw_window, null_mut());
            glfwDestroyWindow(self.glfw_window);

            todo!() // drop(Box::from_raw(ptr as *mut Sender<Event>));
        }
    }
}

impl fmt::Debug for Window {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Window").finish()
    }
}

unsafe extern "C" fn handle_glfw_cursor_pos(w: GlfwWindow, x: c_double, y: c_double) {
    send_event(w, Event::CursorPos(x as _, y as _));
}

unsafe extern "C" fn handle_glfw_scroll(w: GlfwWindow, x: c_double, y: c_double) {
    send_event(w, Event::Scroll(x as _, y as _));
}

unsafe extern "C" fn handle_glfw_mouse_button(w: GlfwWindow, _button: c_int, action: c_int, _mods: c_int) {
    send_event(
        w,
        match action {
            GLFW_PRESS => Event::MouseDown,
            GLFW_RELEASE => Event::MouseUp,
            _ => unreachable!("press/release expected"),
        },
    );
}

unsafe extern "C" fn handle_glfw_key(w: GlfwWindow, key: c_int, _scancode: c_int, action: c_int, _mods: c_int) {
    let which = key_code(key);

    send_event(
        w,
        match action {
            // TODO: repeat works for some keys but for some it doesn't
            // not sure if it's specific for mac (special chars overlay)
            GLFW_RELEASE => Event::KeyUp(which),
            _ => Event::KeyDown(which),
        },
    );
}

unsafe extern "C" fn handle_glfw_char(w: GlfwWindow, char: c_uint) {
    send_event(w, Event::KeyPress(char));
}

unsafe extern "C" fn handle_glfw_window_size(w: GlfwWindow, width: c_int, height: c_int) {
    send_event(w, Event::Resize(width as _, height as _));
}

unsafe extern "C" fn handle_glfw_framebuffer_size(w: GlfwWindow, width: c_int, height: c_int) {
    send_event(w, Event::FramebufferSize(width as _, height as _));
}

unsafe extern "C" fn handle_glfw_window_close(w: GlfwWindow) {
    send_event(w, Event::Close);
}

unsafe fn send_event(win: GlfwWindow, event: Event) {
    // let sender = &*(glfwGetWindowUserPointer(win) as *const Sender<_>);

    // sender.send(event).unwrap();
}

// from GLFW to JS `event.which`
// TODO: modifier (left/right for shift/ctrl/alt)
fn key_code(key: c_int) -> u32 {
    (match key {
        // some codes are the same
        32 | 48..=57 | 65..=90 => key,
        91..=93 => key + 128,

        44 => 188,
        45 => 187,
        46 => 190,
        47 => 189,
        59 => 186,
        61 => 187,
        256 => 27,
        257 => 13,
        258 => 9,
        259 => 8,
        262 => 39,
        263 => 37,
        264 => 40,
        265 => 38,

        // TODO: -1, APOSTROPHE, GRAVE_ACCENT, WORLD1, WORLD2, INSERT, DELETE,
        //   PAGE_UP, PAGE_DOWN, HOME, END, CAPS_LOCK, SCROLL_LOCK, NUM_LOCK
        //   PRINT_SCREEN, PAUSE, F1-F25, KP0-KP9, KP_*
        //   LEFT/RIGHT_SHIFT/CONTROL/ALT
        _ => 0,
    }) as u32
}
