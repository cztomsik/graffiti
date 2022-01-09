use graffiti::{App, Document, Renderer, Window};

fn main() {
    let app = unsafe { App::init() };
    let win = Window::new("Hello", 1024, 768);

    let mut doc = Document::new();
    let div = doc.create_element("div");
    doc[div].el_mut().set_attribute("style", "width: 100px; height: 100px; background: #f00");
    doc.append_child(doc.root(), div);

    let mut renderer = Renderer::new(doc, &win);

    while !win.should_close() {
        renderer.render();
        app.tick();
    }
}
