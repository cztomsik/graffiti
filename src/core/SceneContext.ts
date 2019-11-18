import { send, ApiMsg, AlignProp, Align, FlexDirection, FlexWrap, DimensionProp } from './nativeApi'

/**
 * Provides indirect mutation api for the scene, so that we can freely change an
 * actual message format in the future
 *
 * Operations are batched and not sent until the `flush()` is called
 */
export class SceneContext {
  // because root is 0
  nextId = 1
  changes = []

  constructor(private windowId) {}

  createSurface() {
    this.changes.push([0])
    return this.nextId++
  }

  insertAt(parent, child, index) {
    this.changes.push([1, parent, child, index])
  }

  removeChild(parent, child) {
    this.changes.push([2, parent, child])
  }

  setDimension(surface, prop, dim) {
    this.changes.push([3, surface, DimensionProp[prop], dim])
  }

  setAlign(surface, prop, align) {
    this.changes.push([4, surface, AlignProp[prop], Align[align]])
  }

  setFlexWrap(surface, flexWrap) {
    this.changes.push([5, surface, FlexWrap[flexWrap]])
  }

  setFlexDirection(surface, flex_direction) {
    this.changes.push([6, surface, FlexDirection[flex_direction]])
  }

  setBackgroundColor(surface, color) {
    this.changes.push([7, surface, color])
  }

  setTextColor(surface, color) {
    this.changes.push([8, surface, color])
  }

  setText(surface, text) {
    this.changes.push([9, surface, text])
  }

  flush(animating) {
    if (this.changes.length === 0) {
      return
    }

    //console.log(require('util').inspect(this.msg, { depth: 4 }))
    send(ApiMsg.UpdateScene(this.windowId, this.changes))

    this.changes.length = 0
  }
}
