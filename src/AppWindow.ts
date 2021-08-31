import { native, register, getNativeId } from './native'
import { ERR, Worker } from './util'

export class AppWindow {
  #worker?: Worker
  #send = ERR.bind('no worker')

  constructor({ title = 'Graffiti', width = 1024, height = 768 } = {}) {
    register(this, native.Window_new(title, width, height))
  }

  get width() {
    return native.Window_width(getNativeId(this))
  }

  get height() {
    return native.Window_height(getNativeId(this))
  }

  get title() {
    return native.Window_title(getNativeId(this))
  }

  set title(title: string) {
    native.Window_set_title(getNativeId(this), title)
  }

  show() {
    native.Window_show(getNativeId(this))
  }

  hide() {
    native.Window_hide(getNativeId(this))
  }

  focus() {
    native.Window_focus(getNativeId(this))
  }

  minimize() {
    native.Window_minimize(getNativeId(this))
  }

  maximize() {
    native.Window_maximize(getNativeId(this))
  }

  restore() {
    native.Window_restore(getNativeId(this))
  }

  async loadURL(url: URL | string, options = {}) {
    this.#worker?.terminate()

    const worker = new Worker(new URL('worker.js', import.meta.url), {
      type: 'module',
      deno: true,
    } as any)

    let current = Promise.resolve()
    let next

    worker.addEventListener('message', ({ data }) => {
      if (data?.type === '__GFT') {
        if (data.error) {
          return next.reject(data.error)
        }

        // BTW: can be undefined (key will be missing)
        return next.resolve(data.result)
      }
    })

    // setup sequential req/res communication
    // TODO: prefix or isolate entirely, not sure yet
    this.#worker = worker
    this.#send = msg =>
      (current = current.then(() => {
        next = null
        worker.postMessage(msg)
        return new Promise((resolve, reject) => (next = { resolve, reject }))
      }))

    const [width, height] = [this.width, this.height]
    await this.#send({ type: 'init', windowId: getNativeId(this), width, height, url: '' + url, options })
  }

  async eval(js: string) {
    return this.#send({ type: 'eval', js })
  }
}
