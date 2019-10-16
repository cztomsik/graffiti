import * as os from 'os';

// require() would make ncc bundle some unnecessary build artifacts
process['dlopen'](module, `${__dirname}/../../libgraffiti/target/libgraffiti.node`)

// alloc some mem for result
const resBuf = Buffer.alloc(2 * 1024)

export const send = (msg) => {
  // console.log('send', require('util').inspect(msg, { depth: 4 }))

  // fill with spaces (because of JSON)
  resBuf.fill(0x20)

  // prepare buffer with msg
  const buf = Buffer.from(JSON.stringify(msg))

  // send (sync)
  exports['nativeSend'](buf, resBuf)

  const res = JSON.parse(resBuf.toString('utf-8'))

  if (res.error) {
    throw new Error(res.error)
  }

  return res
}
