// npm run codegen
// cargo build
// npx ts-node test.ts

import { mkColor, Msg, mkMsgHandleEvents, mkMsgAlloc, mkMsgSetBackgroundColor, mkMsgRender } from '../src/astToTs/generated/example'

const ref = require('ref');
const ffi = require('ffi');

// define lib
const libnode_webrender = ffi.Library('./target/debug/libnode_webrender', {
  init: ['void', []],
  // pass a buffer (pointer to some memory + its length)
  'send': ['void', [ref.refType(ref.types.void), 'int']]
});

libnode_webrender.init()

// necessary for window to stay responsive
setInterval(() => {
  send(mkMsgHandleEvents())
  send(mkMsgRender({ surface: 0 }))
}, 100)

send(mkMsgAlloc())
send(mkMsgSetBackgroundColor({ surface: 0, color: mkColor(255, 0, 127, 255) }))

function send(msg: Msg) {
  // prepare buffer with msg
  let buf = Buffer.from(JSON.stringify(msg))

  // send (sync)
  libnode_webrender.send(buf, buf.length)
}
