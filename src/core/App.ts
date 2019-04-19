import { WindowId, FfiMsg, FfiResult, Event } from "./generated";
import { Window } from "./Window";
import * as ffi from './nativeApi'

export class App {
  windows: { [k: number]: Window } = {}
  animating = false
  animationFrames: Function[] = []

  constructor(private ffi) {}

  createWindow() {
    const res = this.ffi.send(FfiMsg.CreateWindow)

    const id = res.value
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
        if (event.tag === 'WindowEvent') {
          this.windows[event.value.window].handleEvent(event.value.event)
        }
      }

      if (this.animating = this.animationFrames.length > 0) {
        const frames = this.animationFrames
        this.animationFrames = []

        for (const cb of frames) {
          cb()
        }
      }

      // TODO: inactive windows could be throttled, maybe even stopped
      // but we should keep HMR working (update in inactive window)
      for (const windowId in this.windows) {
        this.windows[windowId].getSceneContext().flush()
      }

      setTimeout(runLoop)
    }

    runLoop()
  }

  getEvents(): Event[] {
    const res = this.ffi.send(FfiMsg.GetEvents(this.animating))

    if (res.tag === 'Events') {
      return res.value
    }
  }

  requestAnimationFrame(cb) {
    this.animationFrames.push(cb)
  }
}

// lazy created (and shared) App instance
export const APP: App = new Proxy({}, {
  get(holder: any, property) {
    if (!holder.INSTANCE) {
      ffi.init()
      const app = holder.INSTANCE = new App(ffi)
      global['requestAnimationFrame'] = app.requestAnimationFrame.bind(app)
    }

    return holder.INSTANCE[property]
  },

  // needed for method calls
  set(holder, property, value) {
    return Reflect.set(holder.INSTANCE, property, value)
  }
})
