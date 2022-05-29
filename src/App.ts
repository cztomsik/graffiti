import { EventTarget } from './events/EventTarget'
import { native } from './native'

class App extends EventTarget {
  constructor() {
    super()

    native.gft_App_init()
  }

  // TODO
  // quit() {}
}

export const app = new App()

const loop = () => {
  native.gft_App_tick()

  // macro-task, we want to let others run too
  // TODO: should be 0 but this makes WPT run much faster
  //setTimeout(loop, 1)
  setTimeout(loop, 1000)
}

loop()
