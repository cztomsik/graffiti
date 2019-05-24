import { UpdateSceneMsg as U, FfiMsg, StyleProp } from './generated'
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

  setStyleProp(surface, prop: StyleProp) {
    this.sceneMsgs.push(U.SetStyleProp({ surface, prop }))
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
