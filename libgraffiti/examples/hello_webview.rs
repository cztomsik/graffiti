use graffiti::{App, Window, WebView};

fn main() {
    let app = unsafe { App::init() };

    let w = Window::new("Hello WebView", 640, 480);
    let webview = WebView::new();

    webview.attach(&w);
    webview.load_url("https://github.com/cztomsik/graffiti");

    while !w.should_close() {
        app.wait_events();
    }
}
