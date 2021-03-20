import { native } from './native'

export const ID = Symbol()

export class AppWindow {
  #id: number
  #worker?: Worker

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
    const worker = (this.#worker = new Worker(new URL('worker.js', import.meta.url), {
      type: 'module',
      deno: true,
    } as any))

    worker.postMessage({ type: 'init', windowId: this.#id, url: '' + url })

    return new Promise(resolve => worker.addEventListener('message', e => resolve(e.data)))
  }

  async eval(js: string) {
    if (!this.#worker) {
      throw new Error('no worker')
    }

    return new Promise(resolve => {
      // TODO: id
      this.#worker?.postMessage({ type: 'eval', js })
      // TODO: type, id
      this.#worker?.addEventListener('message', e => resolve(e.data), { once: true })
    })
  }
}

const WINDOW_REGISTRY = new FinalizationRegistry(id => native.window_free(id))
