// npm run codegen
// cargo build
// npx ts-node test.ts

import { mkColor, Msg, mkSurfaceId, mkVector2f } from '../src/astToTs/generated/example'

const ref = require('ref');
const ffi = require('ffi');

// define lib
const libnode_webrender = ffi.Library('./target/debug/libnode_webrender', {
  // pass a buffer (pointer to some memory + its length)
  'send': ['void', [ref.refType(ref.types.void), 'int']]
});

send({
  tag: 'SurfaceMsg',
  value: {
    surface: mkSurfaceId(0),
    msg: {
      tag: 'SetSize',
      value: [
        { tag: 'Point', value: 100 },
        { tag: 'Point', value: 100 }
      ]
   }
  }
})

send({
  tag: 'SurfaceMsg',
  value: {
    surface: mkSurfaceId(0),
    msg: {
      tag: 'SetBackgroundColor',
      value: mkColor(0, 0, 0, 1)
    }
  }
})

send({
  tag: 'SurfaceMsg',
  value: {
    surface: mkSurfaceId(0),
    msg: {
      tag: 'SetBoxShadow',
      value: {
        color: mkColor(0, 0, 0, 0.5),
        offset: mkVector2f(2, 2),
        blur: 10,
        spread: -2
      }
    }
  }
})

function send(msg: Msg) {
  // prepare buffer with msg
  let buf = Buffer.from(JSON.stringify(msg))

  // send (sync)
  libnode_webrender.send(buf, buf.length)
}
