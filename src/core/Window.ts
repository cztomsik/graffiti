import * as net from 'net'
import { Container } from './types'
import { Surface, TextContainer } from '.'
import { NativeApi, N } from './nativeApi'
import ResourceManager, { BridgeBrush, BridgeClip } from './ResourceManager'
const native: NativeApi = require('../../native')

  // TODO: HDPI support
;(process as any).env.WINIT_HIDPI_FACTOR = 1

class Window implements Container<Surface | TextContainer> {
  ref: N.Window
  root = new Surface()
  width
  height
  onClose = DEFAULT_CLOSE
  onKeyPress = e => {}

  constructor(title, width = 800, height = 600) {
    this.ref = native.window_create(
      title,
      width,
      height,
      readEvents(e => this.handleEvent(e))
    )

    windowCount++

    ResourceManagerHack(this)

    // TODO: listen for changes
    this.width = width
    this.height = height

    this.root.update({ layout: FILL_LAYOUT })
  }

  // TODO
  handleEvent(event) {
    switch (event.type) {
      case 'Close':
        return this.onClose()
      case 'KeyPress':
        return this.onKeyPress(event)
      case 'Resize':
        this.width = event.width
        this.height = event.height
        this.render()
        return
      case 'Click':
        __callbacks[event.callbackIndex]()
        return
      default:
        throw new Error('unknown event')
    }
  }

  appendChild(child) {
    this.root.appendChild(child)
  }

  insertBefore(child, before) {
    this.root.insertBefore(child, before)
  }

  removeChild(child) {
    this.root.removeChild(child)
  }

  render() {
    return native.window_render_surface(
      this.ref,
      this.root.ref,
      this.width,
      this.height
    )
  }
}

let windowCount = 0

const DEFAULT_CLOSE = () => {
  // TODO: open/close()?
  if (!--windowCount) {
    process.exit()
  }
}

// TODO: optimize
const readEvents = handler => {
  const EVENT_SIZE = 12
  const socketPath = `/tmp/nwr-${Date.now()}`

  const server = net.createServer(socket => {
    let rest = Buffer.alloc(0)

    socket.on('data', b => {
      b = Buffer.concat([rest, b])

      let i = 0

      while (i + EVENT_SIZE <= b.length) {
        handler(decodeEvent(b.slice(i, i + EVENT_SIZE)))
        i += EVENT_SIZE
      }

      rest = b.slice(i)
    })

    console.debug('listening for events')
  })
  server.listen(socketPath)
  return socketPath
}

const decodeEvent = (b: Buffer) => {
  const TYPES = ['Close', 'Resize', 'KeyPress', 'Click']

  const type = TYPES[b.readInt8(0)]

  switch (type) {
    case 'Close':
      return { type }
    case 'Resize':
      return { type, width: b.readFloatLE(4), height: b.readFloatLE(8) }
    case 'KeyPress':
      return { type, ch: String.fromCharCode(b.readInt32LE(4)) }
    case 'Click':
      return { type, callbackIndex: b.readUInt32LE(4) }
    default:
      throw new Error(`unknown event ${type} ${b}`)
  }
}

const handleEvents = () => {
  native.app_loop_a_bit()

  setImmediate(handleEvents)
}

handleEvents()

// TODO: refactor rust so ResourceManager is separate from Window
// (almost done, we just need to get glyph indices & advances)
function ResourceManagerHack(window) {
  WINDOW_HACK = window
}
export let WINDOW_HACK = null

export const __callbacks = []

const FILL_LAYOUT = ResourceManager.getLayout({ width: '100%', height: '100%' })

export default Window
