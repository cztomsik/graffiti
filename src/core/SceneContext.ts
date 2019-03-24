import { UpdateSceneMsg as U, FfiMsg } from './generated'
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
  // TODO: consider ordering related things together (structural, layout, visual changes)
  sceneMsgs = []
  parents = []

  constructor(private windowId) {}

  createSurface() {
    this.sceneMsgs.push(U.Alloc)
    this.parents[this.nextId] = 0
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
    this.sceneMsgs.push(U.SetSize({ surface, size }))
  }

  setFlex(surface, flex) {
    this.sceneMsgs.push(U.SetFlex({ surface, flex }))
  }

  setFlow(surface, flow) {
    this.sceneMsgs.push(U.SetFlow({ surface, flow }))
  }

  setPadding(surface, padding) {
    this.sceneMsgs.push(U.SetPadding({ surface, padding }))
  }

  setMargin(surface, margin) {
    this.sceneMsgs.push(U.SetMargin({ surface, margin }))
  }

  setBorderRadius(surface, borderRadius) {
    this.sceneMsgs.push(U.SetBorderRadius({ surface, borderRadius }))
  }

  setBoxShadow(surface, boxShadow) {
    this.sceneMsgs.push(U.SetBoxShadow({ surface, boxShadow }))
  }

  setBackgroundColor(surface, color) {
    this.sceneMsgs.push(
      U.SetBackgroundColor({
        surface,
        color
      })
    )
  }

  setImage(surface, image) {
    this.sceneMsgs.push(U.SetImage({ surface, image }))
  }

  setText(surface, text) {
    this.sceneMsgs.push(U.SetText({ surface, text }))
  }

  setBorder(surface, border) {
    this.sceneMsgs.push(U.SetBorder({ surface, border }))
  }

  flush() {
    send(
      FfiMsg.UpdateScene({
        window: this.windowId,
        msgs: this.sceneMsgs
      })
    )
    this.sceneMsgs = []
  }
}
