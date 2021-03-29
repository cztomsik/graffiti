// bindings for deno & nodejs
// - thread-local storage, shared fns (this file)
// - submodules define macros and then call export_api!() which is defined here

use crate::backend::{GlBackend, RenderBackend};
use crate::render::Renderer;
use crate::util::SlotMap;
use crate::{App, Document, Event, Rect, WebView, Window};
use crossbeam_channel::{unbounded as channel, Receiver, Sender};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::rc::Rc;

type Task = Box<dyn FnOnce() + 'static + Send>;
static TASK_CHANNEL: Lazy<(Sender<Task>, Receiver<Task>)> = Lazy::new(channel);

static EVENTS: Lazy<DashMap<WindowId, Receiver<Event>>> = Lazy::new(DashMap::new);

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

        export! {
            init: || ctx!().app = Some(unsafe { App::init() }),
            tick: || {
                for task in TASK_CHANNEL.1.try_iter() {
                    task();
                }

                ctx!().app.as_ref().unwrap().wait_events_timeout(0.1);
            },
            render: |w: u32, d: u32| {
                println!("TODO: render {} {}", w, d);

                // TODO: keep it (in TLS)
                let mut renderer = Renderer::new();

                let frame = renderer.render(&ctx!().documents[d], &|_| Rect { pos: (0., 0.), size: (100., 100.) });

                let _ = TASK_CHANNEL.0.send(Box::new(move || {
                    // TODO: keep it somewhere (in main thread?)
                    let mut backend = GlBackend::new();

                    backend.render_frame(frame);

                    ctx!().windows[w].swap_buffers();
                }));
            },

            window_new: |title: String, width, height| {
                let mut w = Window::new(ctx!().app.as_ref().unwrap(), &title, width, height);

                // TODO: make window context current
                unsafe {
                    GlBackend::load_with(|s| w.get_proc_address(s) as _);
                }

                ctx!().windows.insert(w)
            },
            window_next_event: |_w: u32| None::<String>,
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
            window_drop: |w| drop(ctx!().windows.remove(w)),

            webview_new: || {
                let wv = WebView::new(ctx!().app.as_ref().unwrap());
                ctx!().webviews.insert(wv)
            },
            webview_attach: |wv, w| CTX.with(|ctx| {
                let Ctx { ref mut webviews, ref mut windows, .. } = *ctx.borrow_mut();
                webviews[wv].attach(&mut windows[w]);
            }),
            webview_load_url: |wv, url: String| ctx!().webviews[wv].load_url(&url),
            webview_eval: |wv, js: String| ctx!().webviews[wv].eval(&js),
            webview_drop: |wv| drop(ctx!().webviews.remove(wv)),

            document_new: || ctx!().documents.insert(Document::new()),
            document_node_type: |doc, node| ctx!().documents[doc].node_type(node) as u32,
            document_create_text_node: |doc, text: String| ctx!().documents[doc].create_text_node(&text),
            document_create_comment: |doc, text: String| ctx!().documents[doc].create_comment(&text),
            document_set_cdata: |doc, node, text: String| ctx!().documents[doc].set_cdata(node, &text),
            document_create_element: |doc, local_name: String| ctx!().documents[doc].create_element(&local_name),
            document_attribute: |doc, el, attr: String| ctx!().documents[doc].attribute(el, &attr),
            document_set_attribute: |doc, el, attr: String, text: String| ctx!().documents[doc].set_attribute(el, &attr, &text),
            document_remove_attribute: |doc, el, attr: String| ctx!().documents[doc].remove_attribute(el, &attr),
            document_attribute_names: |doc, el| ctx!().documents[doc].attribute_names(el),
            document_insert_child: |doc, el, child, index: u32| ctx!().documents[doc].insert_child(el, child, index as _),
            document_remove_child: |doc, el, child| ctx!().documents[doc].remove_child(el, child),
            document_query_selector: |doc, node, sel: String| ctx!().documents[doc].query_selector(node, &sel),
            document_query_selector_all: |doc, node, sel: String| ctx!().documents[doc].query_selector_all(node, &sel),
            document_drop_node: |doc, node| ctx!().documents[doc].drop_node(node),
            document_drop: |doc| drop(ctx!().documents.remove(doc))
        }
    }};
}

mod deno;
// TODO: Option<T>
//mod nodejs;
