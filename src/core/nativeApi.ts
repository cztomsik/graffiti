import { Msg, mkMsgAlloc, mkMsgHandleEvents } from "./generated";

const ref = require('ref');
const ffi = require('ffi');

// define lib
const lib = ffi.Library(__dirname + '/../../native-new/target/debug/libnode_webrender', {
  init: ['void', []],
  // pass a buffer (pointer to some memory + its length)
  'send': ['void', [ref.refType(ref.types.void), 'int']]
});

lib.init()
send(mkMsgAlloc())

// necessary for window to stay responsive
setInterval(() => {
  send(mkMsgHandleEvents())
}, 200)

export function send(msg: Msg) {
  // prepare buffer with msg
  let buf = Buffer.from(JSON.stringify(msg))

  // send (sync)
  lib.send(buf, buf.length)
}
