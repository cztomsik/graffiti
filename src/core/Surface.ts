import * as yoga from 'yoga-layout'
import { remove } from './utils'
import { Container } from './types'
import { ResourceManager } from '.'
import { BridgeBrush, BridgeClip } from './ResourceManager'
import { RenderOp } from './RenderOperation'

const native = require('../../native')

// container with a layout and an optional brush
// children should have a layout too
class Surface implements Container<Surface> {
  ref

  constructor() {
    this.ref = native.surface_create()
  }

  appendChild(child) {
    native.surface_append_child(this.ref, child.ref)
  }

  insertBefore(child, before) {
    native.surface_insert_before(this.ref, child.ref, before.ref)
  }

  removeChild(child) {
    native.surface_remove_child(this.ref, child.ref)
  }

  setMeasureFunc(f) {
    native.surface_set_measure_func(this.ref, f)
  }

  markDirty() {
    native.surface_mark_dirty(this.ref)
  }

  calculateLayout(availableWidth, availableHeight) {
    native.surface_calculate_layout(this.ref, availableWidth, availableHeight)
  }

  update({ brush = undefined, clip = undefined, layout = DEFAULT_LAYOUT }) {
    native.surface_update(this.ref, brush, clip, layout)
  }
}

// TODO: this should be just value (it's a function until ResourceManager gets really separated)
const DEFAULT_LAYOUT = ResourceManager.getLayout({})

export default Surface
