import { send } from './nativeApi'
import { Transaction } from './Transaction'
import { FfiMsg, UpdateSceneMsg } from './generated'

export class Window {
  rootSurface = 0
  id = 0

  constructor(title: string, width = 800, height = 600) {
    // TODO: factory (no side-effects in constructor)

    // this is ofc wrong because it can change but it's good enough for now (WindowId variant starts at offset 2)
    this.id = send(FfiMsg.CreateWindow).readUInt16LE(2)

    send(FfiMsg.UpdateScene({ window: this.id, msgs: [UpdateSceneMsg.Alloc] }))

    // TODO (in create/openWindow)
    // it's not constructor's job to perform window allocation and so it shouldn't free either
    // createdWindow.destructor = require('finalize')(window, function() { send(destroyWindow(window.id)) }))
  }

  // TODO: consider if it wouldn't be better to enforce single tx at one time
  // (window.getTransaction() would either return current or create a new one)
  createTransaction() {
    return new Transaction(this.id)
  }

  setSize(width: number, height: number) {
    // TODO (sync)
  }

  close() {
    // TODO (sync)
  }
}

export const __callbacks = []
