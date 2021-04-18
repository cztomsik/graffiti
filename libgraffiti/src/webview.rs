// TODO: win/linux

#[cfg(target_os = "macos")]
#[link(name = "WebKit", kind = "framework")]
extern "C" {}

use crate::{App, Window};
use std::os::raw::c_void;
use std::ptr::null;
use std::rc::Rc;

#[cfg(target_os = "macos")]
use objc::{class, msg_send, rc::StrongPtr, runtime::Object, sel, sel_impl};
#[cfg(target_os = "macos")]
#[allow(non_camel_case_types)]
type id = *mut Object;

pub struct WebView {
    _app: Rc<App>,
    #[cfg(target_os = "macos")]
    webview: StrongPtr,
}

impl WebView {
    pub fn new(app: &Rc<App>) -> Self {
        #[cfg(target_os = "macos")]
        unsafe {
            let cfg: id = msg_send![class!(WKWebViewConfiguration), new];
            let del: id = msg_send![class!(NSObject), alloc];
            let webview: id = msg_send![class!(WKWebView), alloc];
            let () = msg_send![webview, initWithFrame:[0f64, 0., 0., 0.] configuration:cfg];
            let () = msg_send![webview, setUIDelegate: del];

            return Self {
                _app: Rc::clone(app),
                webview: StrongPtr::retain(webview),
            };
        }

        #[cfg(not(target_os = "macos"))]
        Self { _app: Rc::clone(app) }
    }

    pub fn attach(&mut self, window: &mut Window) {
        #[cfg(target_os = "macos")]
        unsafe {
            let ns_window: id = window.native_handle() as _;
            let () = msg_send![ns_window, setContentView:*self.webview];
        }
    }

    // TODO: doesn't work when in separate method (it only works as part of new())
    pub fn load_url(&mut self, url: &str) {
        #[cfg(target_os = "macos")]
        unsafe {
            let url: id = msg_send![class!(NSString), stringWithUTF8String: *c_str!(url)];
            let url: id = msg_send![class!(NSURL), URLWithString: url];
            let req: id = msg_send![class!(NSURLRequest), requestWithURL: url];
            let () = msg_send![*self.webview, loadRequest: req];
        }
    }

    pub fn eval(&mut self, js: &str) {
        println!("eval: {:?}", js);

        // TODO: get result (might be tricky because of main thread queue & possible deadlocks)
        //let (tx, rx) = channel();

        #[cfg(target_os = "macos")]
        unsafe {
            let js: id = msg_send![class!(NSString), stringWithUTF8String: *c_str!(js)];

            // TODO: pass closure & get the result
            let () = msg_send![*self.webview, evaluateJavaScript:js completionHandler:null::<*const c_void>()];
        }
    }
}
