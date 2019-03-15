import { FfiMsg, mkFfiMsgHandleEvents } from "./generated";

const ref = require('ref');
const ffi = require('ffi');

// define lib
const lib = ffi.Library(__dirname + '/../../native-new/target/debug/libnode_webrender', {
  init: ['void', []],
  // pass a buffer (pointer to some memory + its length)
  'send': ['void', [ref.refType(ref.types.void), 'int', ref.refType(ref.types.void)]]
});

lib.init()

// necessary for window to stay responsive
setInterval(() => {
  send(mkFfiMsgHandleEvents())
}, 200)

export function send(msg: FfiMsg) {
  // alloc some mem for result
  const result = Buffer.alloc(16)

  // prepare buffer with msg
  let buf = Buffer.from(JSON.stringify(msg))

  console.log(msg)

  // send (sync)
  lib.send(buf, buf.length, result)

  console.log(result)
  return result
}
