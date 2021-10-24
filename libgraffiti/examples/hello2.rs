use graffiti::{App, CharacterDataRef, DocumentRef, ElementRef, NodeRef, NodeType, WebView, Window};

fn main() {
    let app = unsafe { App::init() };
    let win = Window::new("Hello", 400, 300);

    let win2 = Window::new("Debug", 400, 300);
    let webview = WebView::new();
    webview.attach(&win2);

    let doc = DocumentRef::new();
    let h1 = doc.create_element("h1");
    let hello = doc.create_text_node("Hello");
    h1.append_child(&hello);
    doc.append_child(&h1);

    while !win.should_close() {
        for e in win.events().try_iter() {
            hello.set_data(&format!("Hello {:#?}", e));
        }

        webview.eval(&format!("document.body.innerHTML = `{}`", html(&doc)));

        app.wait_events()
    }
}

fn html(node: &NodeRef) -> String {
    match node.node_type() {
        NodeType::Document => html(&node.first_child().unwrap()),
        NodeType::Text => node.clone().downcast::<CharacterDataRef>().unwrap().data(),
        NodeType::Element => {
            let el = node.clone().downcast::<ElementRef>().unwrap();
            let attrs = el
                .attribute_names()
                .iter()
                .map(|att| format!(" {}={:?}", &att, el.attribute(att).unwrap()))
                .collect::<String>();
            format!(
                "<{} {}>{}</{}>",
                el.local_name(),
                attrs,
                el.child_nodes().iter().map(html).collect::<String>(),
                el.local_name()
            )
        }
        _ => String::new(),
    }
}
