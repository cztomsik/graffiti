import { send } from './nativeApi'
import { Transaction } from './Transaction'
import { WindowId, WindowEvent } from './generated'

export class Window {
  rootSurface = 0

  constructor(private id: WindowId) {}

  // TODO: consider if it wouldn't be better to enforce single tx at one time
  // (window.getTransaction() would either return current or create a new one)
  createTransaction() {
    return new Transaction(this.id)
  }

  handleEvent(event: WindowEvent) {
    console.log(event)
  }

  setSize(width: number, height: number) {
    // TODO (sync)
  }

  // TODO (sync)
  // show/hide() - explicit and simple to do
  //
  // it's not clear if close() should just call handler so that app can show
  // confirmation or if it should force the close, etc. let's leave it for later
}

export const __callbacks = []
