import { mkUpdateSceneMsgAppendChild, mkUpdateSceneMsgRemoveChild, mkUpdateSceneMsgInsertBefore, mkFfiMsgUpdateScene } from "./generated";
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
    this.sceneMsgs.push(mkUpdateSceneMsgAppendChild({ parent, child }))
  }

  removeChild(parent, child) {
    this.sceneMsgs.push(mkUpdateSceneMsgRemoveChild({ parent, child }))
  }

  insertBefore(parent, child, before) {
    this.sceneMsgs.push(mkUpdateSceneMsgInsertBefore({ parent, child, before }))
  }

  // not yet sure if it should be here or on window
  _send() {
    send(mkFfiMsgUpdateScene({
      window: this.windowId,
      msgs: this.sceneMsgs
    }))
  }
}
