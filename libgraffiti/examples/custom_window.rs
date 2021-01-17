use graffiti::{backend::GlBackend, Viewport};

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

        let win = glfwCreateWindow(1024, 768, b"Hello\0" as *const _ as _, null_mut(), null_mut());

        glfwMakeContextCurrent(win);

        GlBackend::load_with(|symbol| {
            let symbol = std::ffi::CString::new(symbol).unwrap();
            glfwGetProcAddress(symbol.as_ptr()) as _
        });

        let mut viewport = Viewport::new((1024., 768.), GlBackend::new());

        let doc = viewport.document_mut();
        let h1 = doc.create_element("h1");
        let hello = doc.create_text_node("Hello");
        doc.insert_child(h1, hello, 0);
        doc.insert_child(doc.root(), h1, 0);

        while glfwWindowShouldClose(win) != GLFW_TRUE {
            viewport.update();
            viewport.render();

            glfwSwapBuffers(win);
            glfwWaitEvents();
        }
    }
}
