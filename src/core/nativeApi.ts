import * as ref from 'ref'
import * as ffi from 'ffi'

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

let sink: Sink = {
  arr: new Uint8Array(1024),
  pos: 0
}

const resBuf = Buffer.alloc(1024, 0)

export function send(msg: FfiMsg) {
  //console.log(util.inspect(msg, { depth: 4 }))

  // prepare buffer with msg
  // let msgBuf = Buffer.from(JSON.stringify(msg))

  sink.pos = 0
  sink = writeFfiMsg(sink, msg)

  // this will create just a view on top existing array buffer.
  const msgBuf = Buffer.from(sink.arr.buffer, 0, sink.pos)
  // alloc some mem for result
  // TODO why allocate anything here?

  // send (sync)
  lib.send(msgBuf, msgBuf.length, resBuf)

  const res: FfiResult = readFfiResult({ arr: resBuf, pos: 0 })

  if (res.tag === 'Error') {
    throw new Error(res.value)
  }

  // console.log(res)
  return res
}
