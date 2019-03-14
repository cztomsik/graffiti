import { Transaction } from "./Transaction";

export class Window {
  rootSurface = 0
  id = 0

  constructor(title: string, width = 800, height = 600) {
    // TODO: factory (no side-effects in constructor)

    // TODO (in create/openWindow)
    // it's not constructor's job to perform window allocation and so it shouldn't free either
    // createdWindow.destructor = require('finalize')(window, function() { send(destroyWindow(window.id)) }))
  }

  // TODO: consider if it wouldn't be better to enforce single tx at one time
  // (window.getTransaction() would either return current or create a new one)
  createTransaction() {
    return new Transaction()
  }

  setSize(width: number, height: number) {
    // TODO (sync)
  }

  close() {
    // TODO (sync)
  }
}

export const __callbacks = []
