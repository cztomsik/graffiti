export class AppWindow {
  #id: number
  #worker?: Worker

  constructor(id) {
    this.#id = id
  }

  async loadURL(url) {
    this.#worker?.terminate()

    const Worker = globalThis.Worker ?? (await import('worker_threads')).Worker

    this.#worker = new Worker(new URL('worker.js', import.meta.url), { type: 'module' } as any)
    this.#worker.postMessage({ windowId: this.#id, url })
  }
}
