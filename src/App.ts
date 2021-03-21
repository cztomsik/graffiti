import { native, loadNativeApi } from './native'

export class App {
  private constructor() {}

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
    native.tick()
  }

  static async init() {
    await loadNativeApi()

    native.init()

    return new App()
  }
}
