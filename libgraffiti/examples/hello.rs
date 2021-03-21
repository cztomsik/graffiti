use graffiti::App;

fn main() {
    let app = unsafe { App::init() };
    let mut win = app.create_window("Hello", 400, 300);
    let mut viewport = win.create_viewport();

    let doc = viewport.document_mut();
    let h1 = doc.create_element("h1");
    let hello = doc.create_text_node("Hello");
    doc.insert_child(h1, hello, 0);
    doc.insert_child(doc.root(), h1, 0);

    while !win.should_close() {
        if let Some(e) = win.take_event() {
            viewport.document_mut().set_cdata(hello, &format!("Hello {:#?}", e));
        }

        viewport.update();
        viewport.render();

        win.swap_buffers();
        app.wait_events();
    }
}
