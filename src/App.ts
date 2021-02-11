import { AppWindow } from './AppWindow'
import { loadNativeApi } from './native'
import { ERR } from './util'

export class App {
  #nativeApi

  constructor(nativeApi = ERR('use App.init()')) {
    this.#nativeApi = nativeApi
  }

  createWindow({ title = 'Graffiti', width = 800, height = 600 } = {}) {
    const id = this.#nativeApi.createWindow(title, width, height)

    return new AppWindow(this.#nativeApi, id)
  }

  run() {
    const loop = () => {
      this.tick()

      // macro-task, we want to let others run too
      setTimeout(loop, 0)
    }

    loop()
  }

  // useful for testing/debugging
  tick() {
    this.#nativeApi.tick()
  }

  static async init() {
    const nativeApi = await loadNativeApi()

    nativeApi.init();

    return new App(nativeApi)
  }
}
