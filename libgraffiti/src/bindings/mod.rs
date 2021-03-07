// bindings for deno & nodejs
// - thread-local storage, shared fns (this file)
// - submodules define macros and then include!("api.rs")

use crate::util::SlotMap;
use crate::{App, Document, Window, WebView};
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

macro_rules! ctx {
    () => {
        super::CTX.with(|ctx| ctx.clone()).borrow_mut()
    };
}

mod nodejs;
//mod deno;
