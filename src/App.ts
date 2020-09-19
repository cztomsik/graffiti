import { AppWindow } from './AppWindow'
import { loadNativeApi } from './native'
import { ERR } from './util'

export class App {
  _nativeApi
  _createWorker

  constructor({ nativeApi, createWorker } = ERR('internal, use App.init() instead')) {
    this._nativeApi = nativeApi
    this._createWorker = createWorker
  }

  createWindow({ title = 'Graffiti', width = 800, height = 600 } = {}) {
    const id = this._nativeApi.createWindow(title, width, height)

    return new AppWindow(id, this._createWorker)
  }

  run() {
    const loop = () => {
      this.tick()

      // macro-task, we need to let others run too
      setTimeout(loop, 0)
    }

    loop()
  }

  // useful for testing/debugging
  tick() {
    this._nativeApi.tick()
  }

  static async init() {
    const nativeApi = await loadNativeApi()

    // TODO: import from shared file
    const Worker = globalThis.Worker ?? (await import('worker_threads')).Worker

    const createWorker = () => new Worker(new URL('worker.js', import.meta.url), { type: 'module', deno: true } as any)

    return new App({ nativeApi, createWorker })
  }
}
