import assert from 'assert'
import { Window } from './window/Window'

export const createWindow = ({ title = 'graffiti app', width = 1024, height = 768 } = {}) => {
  return new Window()
}

export const runLoop = () => {
  // get/wait for native events & call listeners
  //
  // - native should poll only if there's something
  //   in libuv loop (nextTick, setImmediate, setTimeout(f, 0)) and
  //   wait for minimum "safe" timeout otherwise (uv_backend_timeout)
  //
  // - any I/O should wake us
  //
  // TODO: it might be good idea to be able to set some minimal
  // wait time just in case there is some native extension with own uv_loop
  native.handleEvents()

  // setTimeout() is too slow but we want to let other handlers fire too
  // so we can't use process.nextTick()
  setImmediate(runLoop)
}

const native = loadNative()

function loadNative() {
  assert(require('worker_threads').isMainThread, 'worker threads are not supported')

  const prevDir = process.cwd()

  // load libs from glfw-dylib if it's present
  // TODO: env might be better idea
  try {
    const LIB_DIR = require('glfw-dylib').DIR
    process.chdir(LIB_DIR)
  } catch (e) {}

  // require() would make ncc bundle some unnecessary build artifacts
  process['dlopen'](module, `${__dirname}/../libgraffiti/target/libgraffiti.node`)

  // restore
  process.chdir(prevDir)

  return exports as any
}
