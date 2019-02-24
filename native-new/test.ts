// npm run codegen
// cargo build
// npx ts-node test.ts

import { mkColor, Msg, mkSurfaceId, mkVector2f } from '../src/astToTs/generated/example'

const ref = require('ref');
const ffi = require('ffi');

// define lib
const libnode_webrender = ffi.Library('./target/debug/libnode_webrender', {
  init: ['void', []],
  // pass a buffer (pointer to some memory + its length)
  'send': ['void', [ref.refType(ref.types.void), 'int']]
});

libnode_webrender.init()

send({ tag: 'Alloc' })

// necessary for window to stay responsive
setInterval(() => send({ tag: 'HandleEvents' }), 30)

function send(msg) {
  // prepare buffer with msg
  let buf = Buffer.from(JSON.stringify(msg))

  // send (sync)
  libnode_webrender.send(buf, buf.length)
}
