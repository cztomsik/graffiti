import { native, register, getNativeId } from './native'

export class App {
  private constructor() {
    register(this, native.gft_App_init())
  }

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
    native.gft_App_tick(getNativeId(this))
  }

  static async init() {
    return new App()
  }
}
