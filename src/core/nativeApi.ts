import { FfiMsg, FfiResult } from './generated'

const ref = require('ref')
const ffi = require('ffi')
const util = require('util')

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

export function send(msg: FfiMsg) {
  //console.log(util.inspect(msg, { depth: 4 }))

  // prepare buffer with msg
  let msgBuf = Buffer.from(JSON.stringify(msg))

  // alloc some mem for result
  const resBuf = Buffer.alloc(1024, ' ')

  // send (sync)
  lib.send(msgBuf, msgBuf.length, resBuf)

  const res: FfiResult = JSON.parse(resBuf.toString('utf-8'))

  //console.log(res)
  return res
}
