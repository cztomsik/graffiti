import { Window } from "../dom/Window";
import { send, ApiMsg } from './nativeApi'
import { performance } from 'perf_hooks'

const windows: Window[] = []
let animating = false
let animationFrames: Function[] = []

export const createWindow = (width = 1024, height = 768) => {
  send(ApiMsg.CreateWindow(width, height))

  // TODO: holes
  const id = windows.length + 1

  return windows[id] = new Window(id)
}

const getEvents = () => {
  return send(ApiMsg.GetEvents(animating)).events
}

// TODO: not yet sure if it should be global or per-window
export const _requestAnimationFrame = (cb) => {
  animationFrames.push(cb)
}

const runLoop = () => {
  // should block if there are no events (with timeout)
  // there might be some results pending in node.js loop so we need to return back (with nothing)
  // would be great if it was possible to access this
  // somehow and only wakeup when necessary (to save cpu/power)
  // maybe we could use async_hooks to know if there was anything requested and if not,
  // just wait indefinitely

  for (const event of getEvents()) {
    /*
    if (event.tag === 'WindowEvent') {
      this.windows[event.value.window].handleEvent(event.value.event)
    }
    */
    windows[1].handleEvent(event)
  }

  if (animating = animationFrames.length > 0) {
    const timestamp = performance.now()
    const frames = animationFrames
    animationFrames = []

    for (const cb of frames) {
      cb(timestamp)
    }
  }

  // TODO: inactive windows could be throttled, maybe even stopped
  // but we should keep HMR working (update in inactive window)
  for (const windowId in windows) {
    windows[windowId].sceneContext.flush(animating)
  }

  // setTimeout is too slow but we want to let other handlers fire too
  setImmediate(runLoop)
  //setTimeout(runLoop, 1000)
}

setTimeout(() => runLoop())
