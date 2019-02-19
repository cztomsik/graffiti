// npm run codegen
// cargo build
// npx ts-node test.ts

const ref = require('ref');
const ffi = require('ffi');

// define lib
const libnode_webrender = ffi.Library('./target/debug/libnode_webrender', {
  // pass a buffer (pointer to some memory + its length)
  'send': ['void', [ref.refType(ref.types.void), 'int']]
});

// prepare buffer with msg
const helloMsg = Buffer.from(JSON.stringify({ tag: 'World', value: "Test" }))

// send (sync)
libnode_webrender.send(helloMsg, helloMsg.length);
