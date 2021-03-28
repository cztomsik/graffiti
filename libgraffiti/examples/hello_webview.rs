use graffiti::{App, Window, WebView};

fn main() {
    let app = unsafe { App::init() };

    let mut w = Window::new(&app, "Hello WebView", 640, 480);
    let mut webview = WebView::new(&app);

    webview.attach(&mut w);
    webview.load_url("https://github.com/cztomsik/graffiti");

    while !w.should_close() {
        app.wait_events();
    }
}
