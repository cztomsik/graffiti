use graffiti::App;

fn main() {
    let mut app = unsafe { App::init() };
    let mut win = app.create_window("Hello", 400, 300);

    let doc = win.viewport_mut().document_mut();
    let h1 = doc.create_element("h1");
    let hello = doc.create_text_node("Hello");
    doc.insert_child(h1, hello, 0);
    doc.insert_child(doc.root(), h1, 0);

    while !win.should_close() {
        if let Some(e) = win.take_event() {
            win.viewport_mut()
                .document_mut()
                .set_text(hello, &format!("Hello {:#?}", e));
        }

        // mozna, ze tohle by mohlo byt na window?
        win.viewport_mut().update();
        win.viewport_mut().render();

        win.swap_buffers();
        app.wait_events();
    }
}
