import { FfiMsg, FfiResult } from './generated'
import { writeFfiMsg } from './serialization.generated'
import { readFfiResult } from './deserialization.generated'
import { Sink } from 'ts-binary'

import * as ref from 'ref'
import * as ffi from 'ffi'

// define lib
const lib = ffi.Library(
  __dirname + '/../../native-new/target/debug/libnode_webrender',
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

  // console.log(res)
  return res
}
