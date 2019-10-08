import * as os from 'os';

// require() would make ncc bundle some unnecessary build artifacts
process['dlopen'](module, `${__dirname}/../../libgraffiti/target/libgraffiti.node`)

export const send = (msg) => {
  // console.log('send', util.inspect(msg, { depth: 4 }))

  // alloc some mem for result
  // fill with spaces (because of JSON)
  const resBuf = Buffer.alloc(1024, 0x20)

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
