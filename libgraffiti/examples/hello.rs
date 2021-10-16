use graffiti::{App, DocumentRef, Renderer, Window};

fn main() {
    let app = unsafe { App::init() };
    let win = Window::new("Hello", 1024, 768);
    let doc = DocumentRef::new();
    let renderer = Renderer::new(doc.clone(), &win);

    let h1 = doc.create_element("h1");
    let hello = doc.create_text_node("Hello");
    h1.append_child(&hello);
    doc.append_child(&h1);

    while !win.should_close() {
        for e in win.events().try_iter() {
            hello.set_data(&format!("Hello {:#?}", e));
        }

        renderer.render();
        app.tick();
    }
}
