use graffiti::{App, CharacterDataRef, DocumentRef, ElementRef, NodeRef, NodeType, WebView, Window};
use std::fmt::Write;

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
    let mut res = String::new();

    match node.node_type() {
        NodeType::Document => {
            write!(res, "{}", html(&node.first_child().unwrap()));
        }
        NodeType::Element => {
            let el = node.clone().downcast::<ElementRef>().unwrap();
            write!(res, "<{}>", el.local_name());

            let mut next = el.first_child();

            while let Some(node) = next {
                next = node.next_sibling();
                write!(res, "{}", html(&node));
            }

            write!(res, "</{}>", el.local_name());
        }
        NodeType::Text => {
            write!(res, "{}", node.clone().downcast::<CharacterDataRef>().unwrap().data());
        }
        _ => {}
    }

    res
}
