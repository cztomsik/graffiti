import { NativeApi, N } from './nativeApi'

type TextContainer = any

class Window {
  ref: N.Window
  width
  height
  onClose = DEFAULT_CLOSE
  onKeyPress = e => {}
  renderScheduled = false

  constructor(title, width = 800, height = 600) {
    //this.ref = native.window_create(title, width, height, evStr =>
    //  this.handleEvent(JSON.parse(evStr))
    //)

    windowCount++

    ResourceManagerHack(this)

    // TODO: listen for changes
    this.width = width
    this.height = height
  }

  // TODO
  handleEvent(event) {
    if (event === 'Close') {
      this.onClose()
      return
    }

    if ('KeyPress' in event) {
      this.onKeyPress({ ch: event.KeyPress })
      return
    }

    if ('Resize' in event) {
      let [width, height] = event.Resize

      this.width = width
      this.height = height
      this.renderLater()
      return
    }

    if ('Click' in event) {
      __callbacks[event.Click]()
      return
    }

    throw new Error('unknown event')
  }

  render() {
  }

  renderLater() {
    if (this.renderScheduled) {
      return
    }

    setImmediate(() => {
      this.render()
      this.renderScheduled = false
    })
    this.renderScheduled = true
  }
}

let windowCount = 0

const DEFAULT_CLOSE = () => {
  // TODO: open/close()?
  if (!--windowCount) {
    process.exit()
  }
}

const handleEvents = () => {
  //native.app_loop_a_bit()

  setImmediate(handleEvents)
}

//handleEvents()

// TODO: refactor rust so ResourceManager is separate from Window
// (almost done, we just need to get glyph indices & advances)
function ResourceManagerHack(window: Window) {
  WINDOW_HACK = window
}
export let WINDOW_HACK: Window = null

export const __callbacks = []

export default Window
