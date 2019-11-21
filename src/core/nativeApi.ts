import * as os from 'os'
export * from './interop'

// require() would make ncc bundle some unnecessary build artifacts
process['dlopen'](module, `${__dirname}/../../libgraffiti/target/libgraffiti.node`)

export const send = (msg) => {
  console.log('send', require('util').inspect(msg, { depth: 4 }))

  // send (sync)
  return exports['nativeSend'](msg)
}
