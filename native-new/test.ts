// depends on `cargo build`
// run with `npx ts-node test.ts`

const ref = require('ref');
const ffi = require('ffi');

// define lib
const libnode_webrender = ffi.Library('./target/debug/libnode_webrender', {
  // pass a buffer (pointer to some memory + its length)
  'send': ['void', [ref.refType(ref.types.void), 'int']]
});

// make a buffer from octets
const helloMsg = Buffer.from([0x0, 0x0, 0x0, 0x0])
const otherMsg = Buffer.from([0x1, 0x0, 0x0, 0x0])

// send (sync)
libnode_webrender.send(helloMsg, helloMsg.length);
libnode_webrender.send(otherMsg, otherMsg.length);
