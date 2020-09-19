// internal (this is the "main script" of each window)

import { loadNativeApi } from './native'
import { Window } from './window/Window'
import { DOMParser } from './dom/DOMParser'
import { createAdapter } from './adapter'

// nodejs
if ('process' in globalThis) {
  import('worker_threads').then(w => w.parentPort?.once('message', main))
} else {
  self.addEventListener('message', e => main(e.data), { once: true })
}

async function main({ windowId, url }) {
  console.log('worker init', windowId, url)

  let nativeApi = await loadNativeApi()

  // get html
  // TODO: file:// urls
  const res = await fetch(url)
  const html = await res.text()

  // create document
  const document: any = new DOMParser(createAdapter(nativeApi, windowId, url)).parseFromString(html, 'text/html')
  document.URL = url

  // create window
  const w = new Window(document)
  Object.setPrototypeOf(globalThis, w)

  const loop = () => nativeApi.takeEvent(windowId, ev => {
    console.log('TODO: dispatch event', ev)

    setTimeout(loop, 0)
  })

  loop()
}
