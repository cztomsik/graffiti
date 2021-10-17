// TODO: win/linux

#[cfg(target_os = "macos")]
#[link(name = "WebKit", kind = "framework")]
extern "C" {}

use std::ffi::CString;
use crate::app::AppOwned;
use crate::{App, Window};
use std::os::raw::c_void;
use std::ptr::null;
use std::sync::Arc;

#[cfg(target_os = "macos")]
use objc::{class, msg_send, rc::StrongPtr, runtime::Object, sel, sel_impl};
#[cfg(target_os = "macos")]
#[allow(non_camel_case_types)]
type id = *mut Object;

pub struct WebView {
    _app: Arc<App>,
    #[cfg(target_os = "macos")]
    webview: AppOwned<StrongPtr>,
}

impl WebView {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let _app = App::current().expect("no App");

        #[cfg(target_os = "macos")]
        return {
            let webview = _app.await_task(|| unsafe {
                let cfg: id = msg_send![class!(WKWebViewConfiguration), new];
                let del: id = msg_send![class!(NSObject), alloc];
                let webview: id = msg_send![class!(WKWebView), alloc];
                let () = msg_send![webview, initWithFrame:[0f64, 0., 0., 0.] configuration:cfg];
                let () = msg_send![webview, setUIDelegate: del];

                AppOwned(StrongPtr::retain(webview))
            });

            Self { _app, webview }
        };

        #[cfg(not(target_os = "macos"))]
        Self { _app }
    }

    pub fn attach(&self, window: &Window) {
        let native_handle = window.native_handle() as usize;

        #[cfg(target_os = "macos")]
        self.webview.with(move |webview| unsafe {
            let ns_window: id = native_handle as _;
            let () = msg_send![ns_window, setContentView:*webview];
        });
    }

    pub fn load_url(&self, url: &str) {
        let url = CString::new(url).unwrap();

        #[cfg(target_os = "macos")]
        self.webview.with(move |webview| unsafe {
            let url: id = msg_send![class!(NSString), stringWithUTF8String: url.as_ptr()];
            let url: id = msg_send![class!(NSURL), URLWithString: url];
            let req: id = msg_send![class!(NSURLRequest), requestWithURL: url];
            let () = msg_send![*webview, loadRequest: req];
        });
    }

    pub fn eval(&self, js: &str) {
        let js = CString::new(js).unwrap();

        println!("eval: {:?}", js);

        // TODO: get result (might be tricky because of main thread queue & possible deadlocks)
        //let (tx, rx) = channel();

        #[cfg(target_os = "macos")]
        self.webview.with(move |webview| unsafe {
            let js: id = msg_send![class!(NSString), stringWithUTF8String: js.as_ptr()];

            // TODO: pass closure & get the result
            let () = msg_send![*webview, evaluateJavaScript:js completionHandler:null::<*const c_void>()];
        })
    }
}
