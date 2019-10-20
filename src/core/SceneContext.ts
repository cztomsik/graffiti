import { send } from './nativeApi'

/**
 * Provides indirect mutation api for the scene, so that we can freely change an
 * actual message format in the future
 *
 * Operations are batched and not sent until the `flush()` is called
 */
export class SceneContext {
  // because root is 0
  nextId = 1
  msg = new UpdateSceneMsg()

  constructor(private windowId) {}

  createSurface() {
    this.msg.tree_changes.push({})
    return this.nextId++
  }

  insertAt(parent, child, index) {
    this.msg.tree_changes.push({ parent, child, index })
  }

  removeChild(parent, child) {
    this.msg.tree_changes.push({ parent, child })
  }

  setText(surface, text) {
    this.msg.text_changes.push({ surface, text })
  }

  setTextColor(surface, color) {
    this.msg.text_changes.push({ surface, color })
  }

  setDimension(surface, prop, dim) {
    this.msg.layout_changes.push({ surface, dim_prop: prop, dim })
  }

  setAlign(surface, prop, align) {
    this.msg.layout_changes.push({ surface, align_prop: prop, align })
  }

  setFlexDirection(surface, flex_direction) {
    this.msg.layout_changes.push({ surface, flex_direction })
  }

  setFlexWrap(surface, flex_wrap) {
    this.msg.layout_changes.push({ surface, flex_wrap })
  }

  setBackgroundColor(surface, color) {
    this.msg.background_color_changes.push({ surface, color })
  }

  flush(animating) {
    if (this.msg.empty) {
      return
    }

    //console.log(require('util').inspect(this.msg, { depth: 4 }))
    send({
      window: this.windowId,
      poll: animating,
      update: this.msg
    })

    this.msg = new UpdateSceneMsg()
  }
}

class UpdateSceneMsg {
  tree_changes = []
  text_changes = []
  layout_changes = []
  background_color_changes = []

  get empty() {
    return ! (this.tree_changes.length || this.text_changes.length || this.layout_changes.length || this.background_color_changes.length)
  }
}

