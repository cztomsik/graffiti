import { UpdateSceneMsg as U, FfiMsg } from './generated'
import { send } from './nativeApi'

/**
 * Represents batch of messages to be sent to native api.
 *
 * Provides (stable) high-level mutation api, so that we can freely change an
 * actual message format in the future
 *
 * (sounds better than SceneMsgBuilder)
 */
export class Transaction {
  // TODO: consider ordering related things together (structural, layout, visual changes)
  sceneMsgs = []

  constructor(private windowId) {}

  appendChild(parent, child) {
    this.sceneMsgs.push(U.AppendChild({ parent, child }))
  }

  removeChild(parent, child) {
    this.sceneMsgs.push(U.RemoveChild({ parent, child }))
  }

  insertBefore(parent, child, before) {
    this.sceneMsgs.push(U.InsertBefore({ parent, child, before }))
  }

  // not yet sure if it should be here or on window
  _send() {
    send(
      FfiMsg.UpdateScene({
        window: this.windowId,
        msgs: this.sceneMsgs
      })
    )
  }
}
