import * as os from 'os'
import { performance } from 'perf_hooks'
export * from './interop'

const prevDir = process.cwd()

// load libs from glfw-dylib if it's present
// TODO: env might be better idea
try {
  const LIB_DIR = require('glfw-dylib').DIR
  process.chdir(LIB_DIR)
} catch (e) {}

// require() would make ncc bundle some unnecessary build artifacts
process['dlopen'](module, `${__dirname}/../../libgraffiti/target/libgraffiti.node`)
const { nativeSend } = exports as any

process.chdir(prevDir)

// everything native-related goes through this
// you just need to create valid message using one of the
// exported factories
export const send = (msg) => {
  //console.log('send', require('util').inspect(msg, { depth: 4 }))

  //const start = performance.now()
  const res = nativeSend(msg)
  //console.log(performance.now() - start)

  return res
}
