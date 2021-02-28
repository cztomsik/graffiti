// TODO: win/linux

use objc::{sel, sel_impl, msg_send, class, runtime::Object, declare::ClassDecl};
type id = *mut Object;

pub struct WebView {
    webview: id
}

impl WebView {
    pub fn new() -> Self {
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
            let () = msg_send![webview, setUIDelegate:del];
    
            //let () = msg_send![ns_window, setContentView:webview];

            Self { webview }
        }
    }

    pub fn loadURL(&self, url: &str) {
        unsafe {
            let url: id = msg_send![class!(NSString), stringWithUTF8String:c_str!(url)];
            let url: id = msg_send![class!(NSURL), URLWithString:url];
            let req: id = msg_send![class!(NSURLRequest), requestWithURL: url];
            let () = msg_send![self.webview, loadRequest: req];
        }
    }

    pub fn eval(&self, js: &str) {
        //let (tx, rx) = channel();

        unsafe {
            let js: id = msg_send![class!(NSString), stringWithUTF8String:c_str!(js)];

            // TODO: pass closure & get the result
            let () = msg_send![self.webview, evaluateJavaScript:js completionHandler:std::ptr::null::<*const ()>()];
        }
    }
}
