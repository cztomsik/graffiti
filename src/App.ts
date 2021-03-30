import { native, loadNativeApi } from './native'

export class App {
  private constructor() {}

  run() {
    const loop = () => {
      this.tick()

      // macro-task, we want to let others run too
      // TODO: should be 0 but this makes WPT run much faster
      setTimeout(loop, 1)
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
