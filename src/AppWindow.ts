export class AppWindow {
  #nativeApi: any
  #id: number
  #worker?: Worker

  constructor(nativeApi, id) {
    this.#nativeApi = nativeApi
    this.#id = id
  }

  // TODO: not sure if this is good (but WebView needs it)
  get id() {
    return this.#id
  }

  get title() {
    return this.#nativeApi.window_title(this.#id)
  }

  set title(title: string) {
    this.#nativeApi.window_set_title(this.#id, title)
  }

  show() {
    this.#nativeApi.window_show(this.#id)
  }

  hide() {
    this.#nativeApi.window_hide(this.#id)
  }

  focus() {
    this.#nativeApi.window_focus(this.#id)
  }

  minimize() {
    this.#nativeApi.window_minimize(this.#id)
  }

  maximize() {
    this.#nativeApi.window_maximize(this.#id)
  }

  restore() {
    this.#nativeApi.window_restore(this.#id)
  }

  async loadURL(url: URL | string) {
    this.#worker?.terminate()

    const Worker = globalThis.Worker ?? (await import('worker_threads')).Worker

    this.#worker = new Worker(new URL('worker.js', import.meta.url), { type: 'module', deno: true } as any)
    this.#worker.postMessage({ type: 'init', windowId: this.#id, scriptUrl: '' + url })
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
