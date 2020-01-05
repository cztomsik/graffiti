import { send, ApiMsg, SceneChange, StyleChange, StyleProp, Align, FlexDirection, FlexWrap } from './nativeApi'

/**
 * Provides indirect mutation api for the scene, so that we can freely change an
 * actual message format in the future
 *
 * Operations are batched and not sent until the `flush()` is called
 */
export class SceneContext {
  // because root is 0
  nextId = 1
  tree_changes = []
  style_changes = []

  constructor(private windowId) {}

  createSurface() {
    this.tree_changes.push(SceneChange.Alloc())
    return this.nextId++
  }

  insertAt(parent, child, index) {
    this.tree_changes.push(SceneChange.InsertAt(parent, child, index))
  }

  removeChild(parent, child) {
    this.tree_changes.push(SceneChange.RemoveChild(parent, child))
  }

  setStyle(surface, prop, value) {
    if (StyleProp[prop]) {
      this.style_changes.push(StyleChange(surface, StyleProp[prop](value)))
    } else {
      console.log('TODO: set', surface, prop, value)
    }
  }

  setDimension(surface, prop, dim) {
    this.setStyle(surface, prop, dim)
  }

  setAlign(surface, prop, align) {
    this.setStyle(surface, prop, Align[align])
  }

  setFlexWrap(surface, wrap) {
    this.setStyle(surface, 'FlexWrap', FlexWrap[wrap])
  }

  setFlexDirection(surface, dir) {
    this.setStyle(surface, 'FlexDirection', FlexDirection[dir])
  }

  setBackgroundColor(surface, color) {
    this.setStyle(surface, 'BackgroundColor', color)
  }

  setTextColor(surface, color) {
    this.setStyle(surface, 'Color', color)
  }

  setText(surface, text) {
    // TODO: this is temporary
    this.setStyle(surface, 'Text', text)
  }

  getBounds(surface) {
    // flush & return quickly
    this.flush(true)

    return send(ApiMsg.GetBounds(this.windowId, surface))[1]
  }

  flush(animating) {
    if (this.tree_changes.length) {
      send(ApiMsg.UpdateScene(this.windowId, this.tree_changes))
      this.tree_changes.length = 0
    }

    if (this.style_changes.length) {
      send(ApiMsg.UpdateStyles(this.windowId, this.style_changes))
      this.style_changes.length = 0
    }
  }
}
