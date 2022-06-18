use graffiti::{App, Document, Viewport};
use std::sync::{Arc, RwLock};

fn main() {
    let mut app = App::init();
    let win = app.create_window("Hello", 400, 300);
    let viewport = Viewport::new((400., 300.), &Arc::new(RwLock::new(parse())));

    app.window_mut(win)
        .set_content(Some(Arc::new(RwLock::new(viewport.into()))));

    loop {
        app.tick()
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
