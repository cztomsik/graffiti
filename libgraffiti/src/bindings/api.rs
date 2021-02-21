// included by both nodejs.rs and deno.rs files
// which both provide different macros so it does slightly different things

// TODO: js_const
// TODO: rename to export and take hash/mapping?
js_module! {
    js_fn!("init", || ctx!().init_app());
    js_fn!("tick", || ctx!().tick());

    js_fn!("window_title", |w| ctx!().windows[w].title().to_owned());
    js_fn!("window_set_title", |w, title: String| ctx!().windows[w].set_title(&title));
    js_fn!("window_show", |w| ctx!().windows[w].show());
    js_fn!("window_hide", |w| ctx!().windows[w].hide());
    js_fn!("window_focus", |w| ctx!().windows[w].focus());
    js_fn!("window_minimize", |w| ctx!().windows[w].minimize());
    js_fn!("window_maximize", |w| ctx!().windows[w].maximize());
    js_fn!("window_restore", |w| ctx!().windows[w].restore());

    js_fn!("document_new", || ctx!().create_document());
    js_fn!("document_create_text_node", |doc, text: String| ctx!().documents[doc].create_text_node(&text));
    js_fn!("document_set_text", |doc, node, text: String| ctx!().documents[doc].set_text(node, &text));
    js_fn!("document_create_element", |doc, local_name: String| ctx!().documents[doc].create_element(&local_name));
    js_fn!("document_set_attribute", |doc, el, attr: String, text: String| ctx!().documents[doc].set_attribute(el, &attr, &text));
    js_fn!("document_remove_attribute", |doc, el, attr: String| ctx!().documents[doc].remove_attribute(el, &attr));
    js_fn!("document_insert_child", |doc, el, child, index: u32| ctx!().documents[doc].insert_child(el, child, index as _));
    js_fn!("document_remove_child", |doc, el, child| ctx!().documents[doc].remove_child(el, child));
}
