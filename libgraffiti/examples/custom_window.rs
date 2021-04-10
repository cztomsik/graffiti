use graffiti::gfx::{GlBackend, RenderBackend};
use graffiti::{Document, Viewport};
use std::cell::RefCell;
use std::rc::Rc;

// or whatever else you use to create a window with GL context
use graffiti_glfw::*;

use std::ptr::null_mut;

fn main() {
    unsafe {
        glfwInit();

        #[cfg(target_os = "macos")]
        {
            glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
            glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 2);
            glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GLFW_TRUE);
            glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
        }

        let (width, height) = (1024, 768);
        let win = glfwCreateWindow(width, height, b"Hello\0" as *const _ as _, null_mut(), null_mut());

        glfwMakeContextCurrent(win);

        GlBackend::load_with(|symbol| {
            let symbol = std::ffi::CString::new(symbol).unwrap();
            glfwGetProcAddress(symbol.as_ptr()) as _
        });

        let document = Rc::new(RefCell::new(Document::new()));
        let mut viewport = Viewport::new((width, height), &document);

        let mut backend = GlBackend::new();

        let mut doc = viewport.document().borrow_mut();
        let root = doc.root();
        let h1 = doc.create_element("h1");
        let hello = doc.create_text_node("Hello");
        doc.insert_child(h1, hello, 0);
        doc.insert_child(root, h1, 0);
        drop(doc);

        while glfwWindowShouldClose(win) != GLFW_TRUE {
            backend.render_frame(viewport.render());

            glfwSwapBuffers(win);
            glfwWaitEvents();
        }
    }
}
