use graffiti::App;

fn main() {
    let app = unsafe { App::init() };

    let mut w = app.create_window("Hello WebView", 640, 480);
    let mut webview = app.create_webview();

    webview.attach(&mut w);
    webview.load_url("https://github.com/cztomsik/graffiti");

    while !w.should_close() {
        app.wait_events();
    }
}
