const native = require('../native')

// see https://github.com/cztomsik/node-webrender/issues/2
const __gcBug = []

class Window extends native.Window {
  constructor(title, width = 800, height = 600) {
    super(title, width, height)
    __gcBug.push(this)

    // TODO: listen for changes
    this.width = width
    this.height = height

    this._freeBuckets = []

    // experimental
    // TODO: do not leak
    this.callbacks = []

    // experimental
    this._consts = {
      TEXT_STACKING_CONTEXT: this.createBucket({
        PushStackingContext: {
          stacking_context: {
            transform_style: 'Flat',
            mix_blend_mode: 'Normal',
            raster_space: 'Screen'
          }
        }
      }),
      POP_STACKING_CONTEXT: this.createBucket({ PopStackingContext: null })
    }

    // keep the process up
    // setInterval(() => {}, Math.pow(2, 17))

    // this is needed just because there is no proper threading now
    setInterval(() => this.handleEvents(), 1000 / 30)
  }

  // experimental
  _setBucket(bucketId, item) {
    if (bucketId === undefined) {
      if (item === undefined) {
        return undefined
      }

      bucketId = this._freeBuckets.pop() || this.createBucket(item)
      return bucketId
    }

    if (item === undefined) {
      this._freeBuckets.push(bucketId)
      return undefined
    }

    this.updateBucket(bucketId, item)
    return bucketId
  }

  createBucket(item) {
    return super.createBucket(JSON.stringify(item))
  }

  updateBucket(bucket, item) {
    super.updateBucket(bucket, JSON.stringify(item))
  }

  render(request) {
    super.render(JSON.stringify(request))
  }

  handleEvents() {
    const callbackIds = new Uint32Array(super.handleEvents())

    for (const i of callbackIds) {
      this.callbacks[i]()
    }
  }

  getGlyphIndicesAndAdvances(str) {
    const [indicesBuffer, advancesBuffer] = super.getGlyphIndicesAndAdvances(str)

    return [new Uint32Array(indicesBuffer), new Float32Array(advancesBuffer)]
  }
}

exports.Window = Window
