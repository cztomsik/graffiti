// API similar to electron BrowserWindow

export class AppWindow {
  #nativeApi: any
  #id: number
  #worker?: Worker

  constructor(nativeApi, id) {
    this.#nativeApi = nativeApi
    this.#id = id
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
    this.#worker.postMessage({ windowId: this.#id, url: '' + url })
  }
}
