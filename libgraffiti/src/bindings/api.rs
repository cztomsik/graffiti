use super::{Ctx, CTX};
use crate::{App, Document};

// included by both nodejs.rs and deno.rs files
// which both provide different macros so it does slightly different things

// BTW: for tuples, we can just accept multiple args and create (a, b, ...) here

#[cfg(target_os = "macos")]
#[link(name = "WebKit", kind = "framework")]
extern "C" {}

// TODO: js_const
// TODO: rename to export and take hash/mapping?
js_module! {
    js_fn!("init", || ctx!().app = Some(unsafe { App::init() }));
    js_fn!("tick", || CTX.with(|ctx| {
        let Ctx { ref mut app, ref mut windows, .. } = *ctx.borrow_mut();
        let app = app.as_mut().expect("no app");

        for (id, win) in windows.iter_mut() {
            if let Some(e) = win.take_event() {
                println!("TODO: {:?}", e);
            }

            win.swap_buffers();
        }

        app.wait_events_timeout(0.1)
    }));

    js_fn!("window_new", |title: String, width, height| CTX.with(|ctx| {
        let Ctx { ref mut app, ref mut windows, .. } = *ctx.borrow_mut();
        let app = app.as_mut().expect("no app");

        windows.insert(app.create_window(&title, width, height))
    }));
    js_fn!("window_title", |w| ctx!().windows[w].title().to_owned());
    js_fn!("window_set_title", |w, title: String| ctx!().windows[w].set_title(&title));
    js_fn!("window_show", |w| ctx!().windows[w].show());
    js_fn!("window_hide", |w| ctx!().windows[w].hide());
    js_fn!("window_focus", |w| ctx!().windows[w].focus());
    js_fn!("window_minimize", |w| ctx!().windows[w].minimize());
    js_fn!("window_maximize", |w| ctx!().windows[w].maximize());
    js_fn!("window_restore", |w| ctx!().windows[w].restore());

    js_fn!("webview_new", || CTX.with(|ctx| {
        let Ctx { ref mut app, ref mut webviews, .. } = *ctx.borrow_mut();
        let app = app.as_mut().expect("no app");

        webviews.insert(app.create_webview())
    }));
    js_fn!("webview_attach", |wv, w| CTX.with(|ctx| {
        let Ctx { ref mut webviews, ref mut windows, .. } = *ctx.borrow_mut();

        webviews[wv].attach(&mut windows[w]);
    }));
    js_fn!("webview_load_url", |wv, url: String| ctx!().webviews[wv].load_url(&url));
    js_fn!("webview_eval", |wv, js: String| ctx!().webviews[wv].eval(&js));

    js_fn!("document_new", || ctx!().documents.insert(Document::new(|_| {})));
    js_fn!("document_create_text_node", |doc, text: String| ctx!().documents[doc].create_text_node(&text));
    js_fn!("document_set_text", |doc, node, text: String| ctx!().documents[doc].set_text(node, &text));
    js_fn!("document_create_element", |doc, local_name: String| ctx!().documents[doc].create_element(&local_name));
    js_fn!("document_set_attribute", |doc, el, attr: String, text: String| ctx!().documents[doc].set_attribute(el, &attr, &text));
    js_fn!("document_remove_attribute", |doc, el, attr: String| ctx!().documents[doc].remove_attribute(el, &attr));
    js_fn!("document_insert_child", |doc, el, child, index: u32| ctx!().documents[doc].insert_child(el, child, index as _));
    js_fn!("document_remove_child", |doc, el, child| ctx!().documents[doc].remove_child(el, child));
}
