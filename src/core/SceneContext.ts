import { send, AppMsg, SceneChange, ElementChild, Align, FlexDirection, FlexWrap, Text, TextAlign, Transform } from './nativeApi'

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
  elementsCount = 1
  textsCount = 0
  changes = []

  // so we can merge allocs together
  reallocMsg = null

  constructor(private windowId) {}

  createElement() {
    const id = this.elementsCount++

    this.realloc()

    return id
  }

  createText() {
    const id = this.textsCount++

    this.realloc()

    return id
  }

  realloc() {
    // update existing
    if (this.reallocMsg !== null) {
      this.reallocMsg[1] = this.elementsCount
      this.reallocMsg[2] = this.textsCount
    } else {
      this.changes.push(this.reallocMsg = SceneChange.Realloc(this.elementsCount, this.textsCount))
    }
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

  // TODO(perf): replace this with own methods
  // or at least accept functions and call them monomorphically
  setStyle(element, prop, value) {
    if (SceneChange[prop]) {
      this.changes.push(SceneChange[prop](element, value))
      //this.flush(false)
    } else {
      console.log('TODO: set', element, prop, value)
    }
  }

  setDimension(element, prop, dim) {
    this.setStyle(element, prop, dim)
  }

  setAlign(element, prop, v) {
    this.setStyle(element, prop, Align[v])
  }

  setFlexWrap(element, v) {
    this.changes.push(SceneChange.FlexWrap(element, FlexWrap[v]))
  }

  setFlexDirection(element, v) {
    this.changes.push(SceneChange.FlexDirection(element, FlexDirection[v]))
  }

  setBackgroundColor(element, v) {
    this.changes.push(SceneChange.BackgroundColor(element, v))
  }

  setColor(element, v) {
    this.changes.push(SceneChange.Color(element, v))
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
    if (this.changes.length) {
      //console.log(this.changes)

      send(AppMsg.UpdateScene(this.windowId, this.changes))
      this.reallocMsg = null
      this.changes.length = 0
    }
  }
}
