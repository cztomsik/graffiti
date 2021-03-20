// bindings for deno & nodejs
// - thread-local storage, shared fns (this file)
// - submodules define macros and then include!("api.rs")

use crate::css::Selector;
use crate::util::SlotMap;
use crate::{App, Document, WebView, Window};
use core::convert::TryFrom;
use std::cell::RefCell;
use std::rc::Rc;

//static VIEWPORTS: Lazy<Mutex<SlotMap<WindowId, Viewport>>> = lazy!(|| Mutex::new(SlotMap::new()));

thread_local! {
    static CTX: Rc<RefCell<Ctx>> = Default::default();
}

type WindowId = u32;
type WebViewId = u32;
type DocumentId = u32;

#[derive(Default)]
struct Ctx {
    app: Option<Rc<App>>,
    windows: SlotMap<WindowId, Window>,
    webviews: SlotMap<WebViewId, WebView>,
    documents: SlotMap<DocumentId, Document>,
}

// Rc<> hack shorthand, TLS.with() is PITA and thread_local crate requires Send
macro_rules! ctx {
    () => {
        super::CTX.with(|ctx| ctx.clone()).borrow_mut()
    };
}

// used in nodejs/deno.rs which both provide different export! macro so it does slightly different things
//
// notes:
// - we are using closure syntax but we only support fn() (deno limitation)
// - finalization order is nondeterministic but it's not a problem for top-level "objects"
//   (key is occupied so reuse can't happen before finalizer is called)
macro_rules! export_api {
    () => {{
        use super::*;

        fn parse_sel(sel: String) -> Selector {
            Selector::try_from(sel.as_str()).unwrap()
        }

        // tuples worked but hinting was pain (generics are pain too but at least this part looks better)
        // https://github.com/cztomsik/graffiti/blob/6637adf0e2fbec4034fb28c770a3fd026a4012c3/libgraffiti/src/bindings/deno.rs
        export! {
            init: || ctx!().app = Some(unsafe { App::init() }),
            tick: || CTX.with(|ctx| {
                let Ctx { ref mut app, ref mut windows, .. } = *ctx.borrow_mut();
                let app = app.as_mut().expect("no app");
                for (id, win) in windows.iter_mut() {
                    if let Some(e) = win.take_event() {
                        println!("TODO: {:?}", e);
                    }
                    win.swap_buffers();
                }
                app.wait_events_timeout(0.1)
            }),

            window_new: |title: String, width, height| CTX.with(|ctx| {
                let Ctx { ref mut app, ref mut windows, .. } = *ctx.borrow_mut();
                let app = app.as_mut().expect("no app");
                windows.insert(app.create_window(&title, width, height))
            }),
            window_title: |w| ctx!().windows[w].title().to_owned(),
            window_set_title: |w, title: String| ctx!().windows[w].set_title(&title),
            window_size: |w| ctx!().windows[w].size(),
            window_set_size: |w, width, height| ctx!().windows[w].set_size((width, height)),
            window_show: |w| ctx!().windows[w].show(),
            window_hide: |w| ctx!().windows[w].hide(),
            window_focus: |w| ctx!().windows[w].focus(),
            window_minimize: |w| ctx!().windows[w].minimize(),
            window_maximize: |w| ctx!().windows[w].maximize(),
            window_restore: |w| ctx!().windows[w].restore(),

            webview_new: || CTX.with(|ctx| {
                let Ctx { ref mut app, ref mut webviews, .. } = *ctx.borrow_mut();
                let app = app.as_mut().expect("no app");
                webviews.insert(app.create_webview())
            }),
            webview_attach: |wv, w| CTX.with(|ctx| {
                let Ctx { ref mut webviews, ref mut windows, .. } = *ctx.borrow_mut();
                webviews[wv].attach(&mut windows[w]);
            }),
            webview_load_url: |wv, url: String| ctx!().webviews[wv].load_url(&url),
            webview_eval: |wv, js: String| ctx!().webviews[wv].eval(&js),

            document_new: || ctx!().documents.insert(Document::new(|_| {})),
            document_node_type: |doc, node| ctx!().documents[doc].node_type(node) as u32,
            document_create_text_node: |doc, text: String| ctx!().documents[doc].create_text_node(&text),
            document_create_comment: |doc, text: String| ctx!().documents[doc].create_comment(&text),
            document_set_cdata: |doc, node, text: String| ctx!().documents[doc].set_cdata(node, &text),
            document_create_element: |doc, local_name: String| ctx!().documents[doc].create_element(&local_name),
            document_set_attribute: |doc, el, attr: String, text: String| ctx!().documents[doc].set_attribute(el, &attr, &text),
            document_remove_attribute: |doc, el, attr: String| ctx!().documents[doc].remove_attribute(el, &attr),
            document_insert_child: |doc, el, child, index: u32| ctx!().documents[doc].insert_child(el, child, index as _),
            document_remove_child: |doc, el, child| ctx!().documents[doc].remove_child(el, child),
            document_query_selector: |doc, node, sel| ctx!().documents[doc].query_selector(node, &parse_sel(sel)),
            document_query_selector_all: |doc, node, sel| ctx!().documents[doc].query_selector_all(node, &parse_sel(sel)),
            document_free_node: |doc, node| ctx!().documents[doc].free_node(node),
            document_free: |doc| { ctx!().documents.remove(doc); }
        }
    }};
}

mod deno;
// TODO: Option<T>
//mod nodejs;
