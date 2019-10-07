import * as os from 'os';

const libDir = (process.env.NODE_ENV === 'production') ?'release' :'debug'
const libSuffix = (os.platform() === 'darwin') ?'dylib' :'so'
process['dlopen'](module, `${__dirname}/../../libgraffiti/target/${libDir}/libgraffiti.${libSuffix}`)

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
