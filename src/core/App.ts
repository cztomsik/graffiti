//import { Window } from "../dom/Window";
//import { send, AppMsg } from './nativeApi'
import { performance } from 'perf_hooks'

// TODO: main thread only
// const { isMainThread } = require('worker_threads')
// assert(isMainThread, '...')

let animating = false
let animationFrames: Function[] = []


export const createWindow = ({ title = 'graffiti app', width = 1024, height = 768 } = {}) => {
  ...
}

// tied to main thread so it has to be shared (& global)
export const requestAnimationFrame = (cb) => {
  animationFrames.push(cb)
}

const runLoop = () => {
  // let native get/wait for events & call listeners
  // (force polling if we are animating)
  //
  // - native should poll only if there's something
  //   in libuv loop (nextTick, setImmediate, setTimeout(f, 0)) and
  //   wait for minimum "safe" timeout otherwise (uv_backend_timeout)
  //
  // - I/O should wake us
  //
  // TODO: it might be good idea to be able to set some minimal
  // wait time just in case there is some native extension with own uv_loop
  native.handleEvents(animating)

  // animate
  if (animating = animationFrames.length > 0) {
    const timestamp = performance.now()
    const frames = animationFrames
    animationFrames = []

    for (const cb of frames) {
      cb(timestamp)
    }
  }

  // TODO: later
  //for (const windowId in windows) {
  //  windows[windowId].sceneContext.flush(animating)
  //}

  // setTimeout is too slow but we want to let other handlers fire too
  setImmediate(runLoop)
}

// defer a bit (let others do some async init too)
setTimeout(() => runLoop, 1)
