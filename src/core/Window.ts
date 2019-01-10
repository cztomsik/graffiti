import * as net from 'net'
import { Container, DrawBrushFunction } from './types'
import { Surface, TextContainer } from '.'
import ResourceManager, {
  BridgeBrush,
  BridgeClip,
  BridgeRect
} from './ResourceManager'
const native = require('../../native')

  // TODO: HDPI support
;(process as any).env.WINIT_HIDPI_FACTOR = 1

class Window extends native.Window
  implements Container<Surface | TextContainer> {
  root = new Surface()
  width
  height
  onClose = DEFAULT_CLOSE
  onKeyPress = e => {}

  constructor(title, width = 800, height = 600) {
    super(title, width, height, readEvents(e => this.handleEvent(e)))

    windowCount++

    ResourceManagerHack(this)

    // TODO: listen for changes
    this.width = width
    this.height = height

    const yn = this.root.yogaNode

    yn.setWidth('100%')
    yn.setHeight('100%')

    // needed because there is no proper threading yet
    setInterval(() => this.handleEvents(), 1000 / 30)
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
    this.root.yogaNode.calculateLayout(this.width, this.height)

    const opResources: (BridgeBrush | BridgeClip)[] = []
    const rects: BridgeRect[] = []

    // TODO it seems there is a potential bug here
    // rust side requires both bucketIds and rects be the same size. Which is might not be the case
    // (it's checked in rust before render)
    const drawBrush: DrawBrushFunction = (brush, rect) => {
      opResources.push(brush)
      rects.push(rect)
    }

    this.root.write(drawBrush, 0, 0)

    // TODO: binary
    // TODO: we convert back and forth from f32 (yoga-cpp, webrender) to f64 (js)
    super.render(JSON.stringify({ op_resources: opResources, layouts: rects }))
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

// TODO: refactor rust so ResourceManager is separate from Window
// (almost done, we just need to get glyph indices & advances)
function ResourceManagerHack(window) {
  WINDOW_HACK = window
}
export let WINDOW_HACK = null

export const __callbacks = []

export default Window
