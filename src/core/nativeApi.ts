import * as os from 'os'
export * from './interop'

// require() would make ncc bundle some unnecessary build artifacts
process['dlopen'](module, `${__dirname}/../../libgraffiti/target/libgraffiti.node`)
const { nativeSend } = exports as any

// everything native-related goes through this
// you just need to create valid message using one of the
// exported factories
//
// @see api.rs
export const send = (msg) => {
  //console.log('send', require('util').inspect(msg, { depth: 4 }))

  return nativeSend(msg)
}
