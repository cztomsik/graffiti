import { mkMsgAppendChild, mkMsgRemoveChild, mkMsgInsertBefore } from "./generated";
import { send } from './nativeApi'

/**
 * Provides (stable) high-level mutation api, so that we can freely change an
 * actual message format in the future
 *
 * (sounds better than SceneMsgBuilder)
 */
export class Transaction {
  // TODO: consider ordering related things together (structural, layout, visual changes)
  sceneMsgs = []

  appendChild(parent, child) {
    this.sceneMsgs.push(mkMsgAppendChild({ parent, child }))
  }

  removeChild(parent, child) {
    this.sceneMsgs.push(mkMsgRemoveChild({ parent, child }))
  }

  insertBefore(parent, child, before) {
    this.sceneMsgs.push(mkMsgInsertBefore({ parent, child, before }))
  }

  // not yet sure if it should be here or on window
  _send() {
    for (const msg of this.sceneMsgs) {
      send(msg)
    }
  }
}
