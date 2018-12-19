import { Container } from './types'
import { Surface, TextContainer } from '.'
import ResourceManager from './ResourceManager'
const native = require('../../native')

// TODO: HDPI support
;(process as any).env.WINIT_HIDPI_FACTOR = 1

class Window extends native.Window implements Container<Surface | TextContainer> {
  root = new Surface()
  width
  height

  constructor(title, width = 800, height = 600) {
    super(title, width, height)

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
  handleEvents() {
    const callbackIds = new Uint32Array(super.handleEvents())

    for (const i of callbackIds) {
      __callbacks[i]()
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

    const bucketIds = []
    const rects = []

    const drawBrush = (brush, rect) => {
      for (const b of brush) {
        bucketIds.push(b)
        rects.push(rect)
      }
    }

    this.root.write(drawBrush, 0, 0)

    // TODO: binary
    // TODO: we convert back and forth from f32 (yoga-cpp, webrender) to f64 (js)
    super.render(JSON.stringify({ bucket_ids: bucketIds, layouts: rects }))
  }
}

// TODO: refactor rust so ResourceManager is separate from Window
function ResourceManagerHack(window) {
  WINDOW_HACK = window

  TEXT_STACKING_CONTEXT = [ResourceManager.createBucket({
    PushStackingContext: {
      stacking_context: {
        transform_style: 'Flat',
        mix_blend_mode: 'Normal',
        raster_space: 'Screen'
      }
    }
  })]
  POP_STACKING_CONTEXT = [ResourceManager.createBucket({ PopStackingContext: null })]
}

export let WINDOW_HACK = null
export let TEXT_STACKING_CONTEXT = null
export let POP_STACKING_CONTEXT = null

export const __callbacks = []

export default Window
