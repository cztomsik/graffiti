import { Window } from "../dom/Window";
import * as ffi from './nativeApi'
import { performance } from 'perf_hooks'

export class App {
  windows: Window[] = []
  animating = false
  animationFrames: Function[] = []

  constructor(private ffi) {}

  createWindow() {
    const res = this.ffi.send({ poll: false })

    // TODO: holes
    const id = this.windows.length + 1
    const window = new Window(id)

    this.windows[id] = window

    return window
  }

  run() {
    const runLoop = () => {
      // should block if there are no events (with timeout)
      // there might be some results pending in node.js loop so we need to return back (with nothing)
      // would be great if it was possible to access this
      // somehow and only wakeup when necessary (to save cpu/power)
      // maybe we could use async_hooks to know if there was anything requested and if not,
      // just wait indefinitely

      for (const event of this.getEvents()) {
        /*
        if (event.tag === 'WindowEvent') {
          this.windows[event.value.window].handleEvent(event.value.event)
        }
        */
        this.windows[1].handleEvent(event)
      }

      if (this.animating = this.animationFrames.length > 0) {
        const timestamp = performance.now()
        const frames = this.animationFrames
        this.animationFrames = []

        for (const cb of frames) {
          cb(timestamp)
        }
      }

      // TODO: inactive windows could be throttled, maybe even stopped
      // but we should keep HMR working (update in inactive window)
      for (const windowId in this.windows) {
        this.windows[windowId].sceneContext.flush(this.animating)
      }

      setTimeout(runLoop)
    }

    runLoop()
  }

  getEvents() {
    // TODO: multi-window
    if (!this.windows[1]) {
      return []
    }

    // TODO: multi-window
    // TODO: poll: this.animating
    return this.ffi.send({ window: 0, poll: this.animating }).events
  }

  requestAnimationFrame(cb) {
    this.animationFrames.push(cb)
  }
}

let APP = undefined

export function getApp({ autoCreate = true, autoRun = true } = {}): App {
  if ((APP === undefined) && autoCreate) {
    APP = new App(ffi)
    global['requestAnimationFrame'] = APP.requestAnimationFrame.bind(APP)

    if (autoRun) {
      APP.run()
    }
  }

  return APP
}
