use graffiti::App;

fn main() {
    let app = unsafe { App::init() };
    let mut w = app.create_window("Hello", 400, 300);
    assert_eq!(w.resizable(), true);
    assert_eq!(w.size(), (400, 300));

    assert_eq!(w.maximized(), false);
    w.maximize();
    assert_eq!(w.maximized(), true);
    w.restore();
    assert_eq!(w.maximized(), false);

    assert_eq!(w.minimized(), false);
    w.minimize();
    assert_eq!(w.minimized(), true);
    w.restore();
    assert_eq!(w.minimized(), false);

    let mut w2 = app.create_window("...", 400, 300);
    w2.set_title("Second window");
    w2.set_size((400, 200));

    assert_eq!(w.visible(), true);
    w2.hide();
    assert_eq!(w2.visible(), false);
    w2.show();
    assert_eq!(w2.visible(), true);
    w2.set_opacity(0.5);
    assert_eq!(w2.opacity(), 0.5);

    w.focus();
    w2.focus();

    w.minimize();
    w.focus();

    while !w.should_close() {
        app.wait_events();
    }
}
