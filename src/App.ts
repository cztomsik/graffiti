import { EventTarget } from './events/EventTarget'
import { send } from './native'

class App extends EventTarget {
  quit() {
    send('Quit')
  }
}

export const app = new App()

const loop = () => {
  send('Tick')

  // macro-task, we want to let others run too
  // TODO: should be 0 but this makes WPT run much faster
  //setTimeout(loop, 1)
  setTimeout(loop, 1000)
}

send('Init')
loop()
