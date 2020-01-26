import { send, AppMsg, SceneChange, ElementChild, Align, FlexDirection, FlexWrap, Text, TextAlign } from './nativeApi'

/**
 * Provides indirect mutation api for the scene, so that we can freely change an
 * actual message format in the future
 *
 * Operations are batched and not sent until the `flush()` is called
 */
export class SceneContext {
  // TODO: freelist, reusing, resetting
  //
  // pooling could improve perf a lot but resetting el/node in a way
  // which is both fast & correct is next to impossible so
  // we can at least reuse the id, send `Reset` msg and avoid
  // the realloc
  //
  // (some GC hook will be needed in `Node`)

  // because root is 0
  nextElementId = 1
  nextTextId = 0
  changes = []

  constructor(private windowId) {}

  createElement() {
    this.changes.push(SceneChange.CreateElement())
    return this.nextElementId++
  }

  createText() {
    this.changes.push(SceneChange.CreateText())
    return this.nextTextId++
  }

  insertElementAt(parent, childElement, index) {
    this.changes.push(SceneChange.InsertAt(parent, ElementChild.Element(childElement), index))
  }

  insertTextAt(parent, childText, index) {
    this.changes.push(SceneChange.InsertAt(parent, ElementChild.Text(childText), index))
  }

  removeElement(parent, childElement) {
    this.changes.push(SceneChange.RemoveChild(parent, ElementChild.Element(childElement)))
  }

  removeText(parent, childText) {
    this.changes.push(SceneChange.RemoveChild(parent, ElementChild.Text(childText)))
  }

  setStyle(element, prop, value) {
    if (SceneChange[prop]) {
      this.changes.push(SceneChange[prop](element, value))
      //this.flush()
    } else {
      console.log('TODO: set', element, prop, value)
    }
  }

  setDimension(element, prop, dim) {
    this.setStyle(element, prop, dim)
  }

  setAlign(element, prop, align) {
    this.setStyle(element, prop, Align[align])
  }

  setFlexWrap(element, wrap) {
    this.setStyle(element, 'FlexWrap', FlexWrap[wrap])
  }

  setFlexDirection(element, dir) {
    this.setStyle(element, 'FlexDirection', FlexDirection[dir])
  }

  setBackgroundColor(element, color) {
    this.setStyle(element, 'BackgroundColor', color)
  }

  setColor(element, color) {
    this.setStyle(element, 'Color', color)
  }

  setText(textId, size, lineHeight, align, text) {
    this.changes.push(SceneChange.SetText(textId, Text(size, lineHeight, align, text)))
  }

  getOffsetBounds(element) {
    // flush & return quickly
    this.flush(true)

    return send(AppMsg.GetOffsetBounds(this.windowId, element))[1]
  }

  flush(animating) {
    //console.log(this.changes)

    if (this.changes.length) {
      send(AppMsg.UpdateScene(this.windowId, this.changes))
      this.changes.length = 0
    }
  }
}
