use graffiti::{App, Document, Event as WindowEvent, Renderer, Viewport, Window};

fn main() {
    let mut app = unsafe { App::init() };
    let mut win = Window::new("Hello", 400, 300);
    // TODO: fb_size might be different and maybe we should not call renderer.resize() from viewport
    let mut viewport = Viewport::new(win.size(), parse(), unsafe {
        win.make_current();
        Renderer::new(win.size())
    });

    while !win.should_close() {
        app.wait_events();

        while let Ok(event) = win.events().try_recv() {
            match event {
                WindowEvent::CursorPos(_x, _y) => {} // viewport.move(x, y)
                // TODO: click, scroll, tab_next/prev, ...
                WindowEvent::Resize(width, height) => viewport.resize((width as _, height as _)),
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
