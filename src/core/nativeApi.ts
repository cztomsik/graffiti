import * as ref from 'ref'
import * as ffi from 'ffi'
import * as util from 'util'

// define lib
const libDir = (process.env.NODE_ENV === 'production') ?'release' :'debug'
const lib = ffi.Library(
  `${__dirname}/../../libgraffiti/target/${libDir}/libgraffiti`,
  {
    init: ['void', []],
    // pass a buffer (pointer to some memory + its length)
    send: [
      'void',
      [ref.refType(ref.types.void), 'int', ref.refType(ref.types.void)]
    ]
  }
)

export const init = () => lib.init()

export const send = (msg) => {
  console.log('send', util.inspect(msg, { depth: 4 }))

  // alloc some mem for result
  // fill with spaces (because of JSON)
  const resBuf = Buffer.alloc(1024, 0x20)

  // prepare buffer with msg
  const buf = Buffer.from(JSON.stringify(msg))

  // send (sync)
  lib.send(buf, buf.length, resBuf)

  const res = JSON.parse(resBuf.toString('utf-8'))

  if (res.error) {
    throw new Error(res.error)
  }

  return res
}
