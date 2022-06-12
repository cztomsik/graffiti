use glfw::{Context, Glfw, Window, WindowEvent};
use graffiti::{Document, Renderer, Viewport};
use std::sync::mpsc::Receiver;

fn main() {
    let (mut glfw, mut win, events) = create_window();
    // TODO: fb_size might be different and maybe we should not call renderer.resize() from viewport
    let mut viewport = Viewport::new(win.get_size(), parse(), Renderer::new(win.get_size()));

    while !win.should_close() {
        glfw.wait_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::CursorPos(_x, _y) => {} // viewport.move(x, y)
                // TODO: click, scroll, tab_next/prev, ...
                WindowEvent::Size(width, height) => viewport.resize((width, height)),
                _ => println!("{:?}", &event),
            }
        }

        viewport.render();
        win.swap_buffers();
    }
}

fn parse() -> Document {
    let mut doc = Document::new();

    let body = doc.create_element("body");
    let h1 = doc.create_element("h1");
    let hello = doc.create_text_node("Hello 🚀");
    let panel = doc.create_element("div");
    let text = doc.create_text_node("Some text which wraps but is too long anyway so it gets clipped.....................................................");

    doc.append_child(Document::ROOT, body);
    doc.append_child(body, h1);
    doc.append_child(h1, hello);
    doc.append_child(body, panel);
    doc.append_child(panel, text);

    doc.set_style(body, "height: 300px; padding: 20px; background-color: #ccf");
    doc.set_style(h1, "padding: 20px; outline-style: solid;");
    doc.set_style(
        panel,
        "padding: 20px; background-color: #fff; border-radius: 10px; overflow: visible; box-shadow: 0 0 10px #000",
    );

    doc
}

fn create_window() -> (Glfw, Window, Receiver<(f64, WindowEvent)>) {
    let glfw = init_glfw();

    let (mut win, events) = glfw
        .create_window(400, 300, "Hello", glfw::WindowMode::Windowed)
        .expect("GLFW create_window()");

    win.make_current();
    win.set_all_polling(true);

    (glfw, win, events)
}

fn init_glfw() -> Glfw {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    glfw
}
