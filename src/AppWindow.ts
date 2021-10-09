import { native, encode, register, getNativeId } from './native'
import { ERR, Worker } from './util'

export class AppWindow {
  #worker?: Worker
  #send: any = ERR.bind('no worker')

  constructor({ title = 'Graffiti', width = 1024, height = 768 } = {}) {
    register(this, native.gft_Window_new(...encode(title), width, height))
  }

  get width() {
    return native.gft_Window_width(getNativeId(this))
  }

  get height() {
    return native.gft_Window_height(getNativeId(this))
  }

  get title() {
    return native.gft_Window_title(getNativeId(this))
  }

  set title(title: string) {
    native.gft_Window_set_title(getNativeId(this), title)
  }

  show() {
    native.gft_Window_show(getNativeId(this))
  }

  hide() {
    native.gft_Window_hide(getNativeId(this))
  }

  focus() {
    native.gft_Window_focus(getNativeId(this))
  }

  minimize() {
    native.gft_Window_minimize(getNativeId(this))
  }

  maximize() {
    native.gft_Window_maximize(getNativeId(this))
  }

  restore() {
    native.gft_Window_restore(getNativeId(this))
  }

  async loadURL(url: URL | string, options = {}) {
    console.log(this.width, this.height)

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
    // TODO: it should now be possible to create channel and send it to the worker
    //       and use that for our two-way communication
    this.#worker = worker
    this.#send = msg => {
      return (current = current.then(() => {
        next = null
        worker.postMessage(msg)
        return new Promise((resolve, reject) => (next = { resolve, reject }))
      }))
    }

    const { width, height } = this
    await this.#send({ type: 'init', windowId: getNativeId(this), width, height, url: '' + url, options })
  }

  async eval(js: string) {
    return this.#send({ type: 'eval', js })
  }
}
