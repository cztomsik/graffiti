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
    return this.#nativeApi.getWindowTitle(this.#id)
  }

  set title(title: string) {
    this.#nativeApi.setWindowTitle(this.#id, title)
  }

  show() {
    this.#nativeApi.showWindow(this.#id);
  }

  hide() {
    this.#nativeApi.hideWindow(this.#id);
  }

  focus() {
    this.#nativeApi.focusWindow(this.#id);
  }

  minimize() {
    this.#nativeApi.minimizeWindow(this.#id);
  }

  maximize() {
    this.#nativeApi.maximizeWindow(this.#id);
  }

  restore() {
    this.#nativeApi.restoreWindow(this.#id);
  }

  async loadURL(url) {
    this.#worker?.terminate()

    const Worker = globalThis.Worker ?? (await import('worker_threads')).Worker

    this.#worker = new Worker(new URL('worker.js', import.meta.url), { type: 'module', deno: true } as any)
    this.#worker.postMessage({ windowId: this.#id, url })
  }
}
