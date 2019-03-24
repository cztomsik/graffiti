import { WindowId, FfiMsg, FfiResult, Event } from "./generated";
import { Window } from "./Window";
import * as ffi from './nativeApi'

export class App {
  windows: Window[] = []

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

      // TODO: in case we were animating, we should just poll
      // (add timeout param to GetNextEvent)
      const event = this.getNextEvent()

      if (event !== undefined) {
        if (event.tag === 'WindowEvent') {
          const window = this.windows[event.value.window]

          window.handleEvent(event.value.event)
        }
      }

      setTimeout(runLoop, 100)
    }

    runLoop()
  }

  getNextEvent(): Event | undefined {
    const res = this.ffi.send(FfiMsg.GetNextEvent)

    if (res.tag === 'Event') {
      return res.value
    }
  }
}

// lazy created (and shared) App instance
export const APP: App = new Proxy({}, {
  get(holder: any, property) {
    if (!holder.INSTANCE) {
      ffi.init()
      holder.INSTANCE = new App(ffi)
    }

    return holder.INSTANCE[property]
  },

  // needed for method calls
  set(holder, property, value) {
    return Reflect.set(holder.INSTANCE, property, value)
  }
})
