use graffiti::gfx::{GlBackend, RenderBackend};
use graffiti::{Document, Node, Viewport};

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

        let doc = Document::new();
        let viewport = Viewport::new((width, height), doc.clone());

        let mut backend = GlBackend::new(|symbol| {
            let symbol = std::ffi::CString::new(symbol).unwrap();
            glfwGetProcAddress(symbol.as_ptr()) as _
        });

        let h1 = doc.create_element("h1");
        let hello = doc.create_text_node("Hello");
        h1.append_child(hello);
        doc.append_child(h1);
        drop(doc);

        while glfwWindowShouldClose(win) != GLFW_TRUE {
            backend.render_frame(viewport.render());

            glfwSwapBuffers(win);
            glfwWaitEvents();
        }
    }
}
