use serde::Deserialize;
use bincode;

use tscodegen::tscodegen;

// nomangle is required for ffi
#[no_mangle]
pub extern "C" fn send(data: *const u8, len: u32) {
    // get slice of bytes & try to deserialize
    let msg = unsafe { std::slice::from_raw_parts(data, len as usize) };
    let msg: Msg = bincode::deserialize(msg).expect("invalid message");

    println!("Got {:?}", msg);
}

// tscodegen has to be first
#[tscodegen]
#[derive(Deserialize, Debug)]
pub enum Msg {
    Hello,
    World
}
