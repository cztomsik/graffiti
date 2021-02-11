// included by both nodejs.rs and deno.rs files
// which both provide different macros so it does slightly different things

// TODO: js_const
// TODO: rename to export and take hash/mapping?
js_module! {
    js_fn!("init", || ctx!().init_app());
    js_fn!("tick", || ctx!().tick());
    js_fn!("createWindow", |title: String, width, height| ctx!().create_window(&title, width, height));
    js_fn!("showWindow", |w| ctx!().windows[w].show());
    js_fn!("hideWindow", |w| ctx!().windows[w].hide());
    js_fn!("focusWindow", |w| ctx!().windows[w].focus());
    js_fn!("minimizeWindow", |w| ctx!().windows[w].minimize());
    js_fn!("maximizeWindow", |w| ctx!().windows[w].maximize());
    js_fn!("restoreWindow", |w| ctx!().windows[w].restore());
}
