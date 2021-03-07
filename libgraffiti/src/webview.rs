// TODO: win/linux

use crate::{App, Window};
use objc::{class, declare::ClassDecl, msg_send, rc::StrongPtr, runtime::Object, sel, sel_impl};
use std::rc::Rc;
#[allow(non_camel_case_types)]
type id = *mut Object;

pub struct WebView {
    app: Rc<App>,
    webview: StrongPtr,
}

impl WebView {
    pub(crate) fn new(app: Rc<App>) -> Self {
        unsafe {
            let cfg: id = msg_send![class!(WKWebViewConfiguration), new];
            // at least on mac, zero works too
            let rect: [f64; 4] = [0., 0., 800., 600.];
            let cls = {
                let cls = ClassDecl::new("WebViewDelegate", class!(NSObject)).unwrap();
                // TODO: handlers
                cls.register()
            };
            let del: id = msg_send![cls, alloc];
            let webview: id = msg_send![class!(WKWebView), alloc];
            let () = msg_send![webview, initWithFrame:rect configuration:cfg];
            let () = msg_send![webview, setUIDelegate: del];

            // TODO: find out what's the reason (see load_url())
            let url: id = msg_send![class!(NSString), stringWithUTF8String: c_str!("https://google.com")];
            let url: id = msg_send![class!(NSURL), URLWithString: url];
            let req: id = msg_send![class!(NSURLRequest), requestWithURL: url];
            let () = msg_send![webview, loadRequest: req];

            Self {
                app,
                webview: StrongPtr::retain(webview),
            }
        }
    }

    pub fn attach(&mut self, window: &mut Window) {
        unsafe {
            let ns_window: id = window.native_handle() as _;
            let () = msg_send![ns_window, setContentView:*self.webview];
        }
    }

    // TODO: doesn't work when in separate method (it only works as part of new())
    pub fn load_url(&mut self, url: &str) {
        // unsafe {
        //     let url: id = msg_send![class!(NSString), stringWithUTF8String: c_str!(url)];
        //     let url: id = msg_send![class!(NSURL), URLWithString: url];
        //     let req: id = msg_send![class!(NSURLRequest), requestWithURL: url];
        //     let () = msg_send![self.webview, loadRequest: req];
        // }
    }

    pub fn eval(&mut self, js: &str) {
        //let (tx, rx) = channel();

        unsafe {
            let js: id = msg_send![class!(NSString), stringWithUTF8String: c_str!(js)];

            // TODO: pass closure & get the result
            let () = msg_send![*self.webview, evaluateJavaScript:js completionHandler:std::ptr::null::<*const ()>()];
        }
    }
}
