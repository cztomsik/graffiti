mod generated;
mod surface;

// TODO: generics
// (into() is not enough; maybe struct)
// usize for now
pub type Id = usize;

mod layout;
mod temp;

/*
use crate::layout::{update_layout, LayoutMsg};
use serde::Deserialize;
use serde_json;
use yoga::{FlexStyle, Node as YogaNode, StyleUnit};

static mut APP: Option<App> = None;

#[no_mangle]
pub extern "C" fn init() {
    temp::init();

    let layout: Vec<YogaNode> = Vec::new();

    unsafe { APP = Some(App { layout }) }
}

#[no_mangle]
pub extern "C" fn send(data: *const u8, len: u32) {
    // get slice of bytes & try to deserialize
    let msg = unsafe { std::slice::from_raw_parts(data, len as usize) };
    let msg: Msg = serde_json::from_slice(msg).expect("invalid message");

    unsafe {
        match APP {
            None => {}
            Some(ref mut app) => match msg {
                Msg::HandleEvents => temp::handle_events(),
                Msg::Alloc => app.layout.push(YogaNode::new()),
                Msg::UpdateLayout(msgs) => update_layout(&mut app.layout, msgs),
            },
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "tag", content = "value")]
enum Msg {
    HandleEvents,
    Alloc,
    UpdateLayout(Vec<LayoutMsg>),
}

struct App {
    layout: Vec<YogaNode>,
}
*/
