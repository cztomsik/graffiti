use graffiti::{App, DocumentRef, Renderer, Window};

fn main() {
    let app = unsafe { App::init() };
    let win = Window::new("Hello", 1024, 768);
    let doc = DocumentRef::new();
    let renderer = Renderer::new(&doc, &win);

    let div = doc.create_element("div");
    div.set_attribute("style", "width: 100px; padding: 100px; height: 100px; background: #f00; border-radius: 10px; border-top-left-radius: 50px");

    let h1 = doc.create_element("h1");
    let hello = doc.create_text_node("Hello");
    h1.append_child(&hello);
    div.append_child(&h1);
    doc.append_child(&div);

    while !win.should_close() {
        for e in win.events().try_iter() {
            hello.set_data(&format!("Hello {:#?}", e));
        }

        renderer.render();
        app.tick();
    }
}
