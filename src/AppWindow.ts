import { Worker } from './util'
import type { WorkerApi } from './worker'
import { send } from './native'

export class AppWindow {
  #id
  #worker?: Worker
  #port?: MessagePort

  constructor({ title = 'Graffiti', width = 1024, height = 768 } = {}) {
    this.#id = send({ CreateWindow: [title, width, height] })

    // REGISTRY.register(this, this.#id)
  }

  async loadURL(url: URL) {
    this.#worker?.terminate()

    // create worker
    this.#worker = new Worker(new URL('worker.js', import.meta.url), {
      type: 'module',
      deno: { namespace: true, permissions: 'inherit' },
    } as any)

    // setup IPC
    const { port1, port2 } = new MessageChannel()
    this.#port = port1
    this.#worker.postMessage(port2, [port2])

    return this.#send('run', this.#id, `${url}`)
  }

  // type-safe, async IPC
  async #send<P extends keyof WorkerApi>(cmd: P, ...args: Parameters<WorkerApi[P]>): Promise<ReturnType<WorkerApi[P]>> {
    return new Promise((resolve, reject) => {
      const { port1: receiver, port2: sender } = new MessageChannel()
      this.#port?.postMessage([cmd, args, sender], [sender])
      receiver.onmessage = ({ data }) => (data.error ? reject(data.error) : resolve(data.result))
    })
  }

  async eval(js: string) {
    return this.#send('eval', js)
  }

  postMessage(msg, options?) {
    this.#worker?.postMessage(msg, options)
  }
}

// const REGISTRY = new FinalizationRegistry(id => native.gft_Window_drop(id))
