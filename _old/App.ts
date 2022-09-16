// TODO: it would be nice if we could get close(r) to electron api
//       https://www.electronjs.org/docs/latest/api/app#methods

import { EventTarget } from './events/EventTarget'
import { send } from './native'

class App extends EventTarget {
  constructor() {
    super()

    send('Init')
  }

  // TODO: focus/show/hide/quit()
}

export const app = new App()

const loop = () => {
  send('Tick')

  // macro-task, we want to let others run too
  // TODO: should be 0 but this makes WPT run much faster
  //setTimeout(loop, 1)
  setTimeout(loop, 300)
}

loop()
