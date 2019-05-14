import { UpdateSceneMsg as U, StyleProp as S, FfiMsg } from './generated'
import { send } from './nativeApi'
import { NotSureWhat } from './events/NotSureWhat';

/**
 * Provides indirect mutation api for the scene, so that we can freely change an
 * actual message format in the future
 *
 * Operations are batched and not sent until the `flush()` is called
 */
export class SceneContext {
  // because root is 0
  nextId = 1
  // TODO: consider ordering related things together (structural, layout, visual changes)
  sceneMsgs = []
  parents = []
  events = new NotSureWhat(this.parents)

  constructor(private windowId) {}

  createSurface() {
    this.sceneMsgs.push(U.Alloc)
    this.parents[this.nextId] = 0
    this.events.alloc()
    return this.nextId++
  }

  appendChild(parent, child) {
    this.sceneMsgs.push(U.AppendChild({ parent, child }))
    this.parents[child] = parent
  }

  removeChild(parent, child) {
    this.sceneMsgs.push(U.RemoveChild({ parent, child }))
    this.parents[child] = 0
  }

  insertBefore(parent, child, before) {
    this.sceneMsgs.push(U.InsertBefore({ parent, child, before }))
    this.parents[child] = parent
  }

  setSize(surface, size) {
    this.sceneMsgs.push(U.SetStyleProp({ surface, prop: S.Size(size) }))
  }

  setOverflow(surface, overflow) {
    this.sceneMsgs.push(U.SetStyleProp({ surface, prop: S.Overflow(overflow) }))
  }

  setFlex(surface, flex) {
    this.sceneMsgs.push(U.SetStyleProp({ surface, prop: S.Flex(flex) }))
  }

  setFlow(surface, flow) {
    this.sceneMsgs.push(U.SetStyleProp({ surface, prop: S.Flow(flow) }))
  }

  setPadding(surface, padding) {
    this.sceneMsgs.push(U.SetStyleProp({ surface, prop: S.Padding(padding) }))
  }

  setMargin(surface, margin) {
    this.sceneMsgs.push(U.SetStyleProp({ surface, prop: S.Margin(margin) }))
  }

  setBorderRadius(surface, borderRadius) {
    this.sceneMsgs.push(U.SetStyleProp({ surface, prop: S.BorderRadius(borderRadius) }))
  }

  setBoxShadow(surface, boxShadow) {
    this.sceneMsgs.push(U.SetStyleProp({ surface, prop: S.BoxShadow(boxShadow) }))
  }

  setBackgroundColor(surface, color) {
    this.sceneMsgs.push(
      U.SetStyleProp({ surface, prop: S.BackgroundColor(color) })
    )
  }

  setImage(surface, image) {
    this.sceneMsgs.push(U.SetStyleProp({ surface, prop: S.Image(image) }))
  }

  setText(surface, text) {
    this.sceneMsgs.push(U.SetStyleProp({ surface, prop: S.Text(text) }))
  }

  setBorder(surface, border) {
    this.sceneMsgs.push(U.SetStyleProp({ surface, prop: S.Border(border) }))
  }

  flush() {
    if (this.sceneMsgs.length === 0) {
      return
    }

    send(
      FfiMsg.UpdateScene({
        window: this.windowId,
        msgs: this.sceneMsgs
      })
    )
    this.sceneMsgs = []
  }
}
