import { native } from './native'
import { ERR } from './util'

export const ID = Symbol()

export class AppWindow {
  #id: number
  #worker?: Worker
  #send = ERR.bind('no worker')

  constructor({ title = 'Graffiti', width = 800, height = 600 } = {}) {
    this.#id = native.window_new(title, width, height)

    WINDOW_REGISTRY.register(this, this.#id)
  }

  // TODO: not sure if this is good (but WebView needs it)
  get [ID]() {
    return this.#id
  }

  get title() {
    return native.window_title(this.#id)
  }

  set title(title: string) {
    native.window_set_title(this.#id, title)
  }

  show() {
    native.window_show(this.#id)
  }

  hide() {
    native.window_hide(this.#id)
  }

  focus() {
    native.window_focus(this.#id)
  }

  minimize() {
    native.window_minimize(this.#id)
  }

  maximize() {
    native.window_maximize(this.#id)
  }

  restore() {
    native.window_restore(this.#id)
  }

  async loadURL(url: URL | string) {
    this.#worker?.terminate()

    const Worker = globalThis.Worker ?? (await import('worker_threads')).Worker
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
    this.#send = async msg => {
      await current
      next = null
      worker.postMessage(msg)
      return (current = new Promise((resolve, reject) => (next = { resolve, reject })))
    }

    await this.#send({ type: 'init', windowId: this.#id, url: '' + url })
  }

  async eval(js: string) {
    return this.#send({ type: 'eval', js })
  }
}

const WINDOW_REGISTRY = new FinalizationRegistry(id => native.window_free(id))
