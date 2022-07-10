// internal (this is the "main script" of each window)
// we need workers because each window needs separate scope

import { Window, makeGlobal } from './window/Window'
import { parseIntoDocument } from './dom/DOMParser'
import { readURL } from './util'
import { loadStyles } from './dom/HTMLLinkElement'
import { runScripts } from './dom/HTMLScriptElement'
import { send } from './native'

// nodejs
if ('process' in globalThis) {
  const { parentPort } = await import('worker_threads')
  globalThis.postMessage = (msg, _) => parentPort?.postMessage(msg)
  parentPort?.once('message', data => init({ data }))
} else {
  self.addEventListener('message', init, { once: true })
}

// setup IPC
function init({ data: port }) {
  const api = new WorkerApi()
  port.onmessage = async ({ data: [cmd, args, res] }) => {
    try {
      res.postMessage({ result: await api[cmd](...args) })
    } catch (error) {
      console.log('err', error)
      res.postMessage({ error })
    }
  }
}

class WorkerApi {
  async run(url) {
    // setup env
    const { window, document } = new Window()
    document.URL = url
    makeGlobal(window)

    // load html
    parseIntoDocument(document, await readURL(url))
    Object.assign(document, { defaultView: window, URL: url })

    // load (once) remote styles
    await loadStyles()

    // run (once) all the scripts
    await runScripts()

    // TODO: we should somehow detect changes and notify parent
    //       so the tick() can skip rendering untouched windows
  }

  handleEvent(event: any) {
    console.log('event', event)
  }

  async eval(code) {
    return eval.call(null, code)
  }
}

export type { WorkerApi }
