// internal (this is the "main script" of each window)

import { loadNativeApi } from './native'
import { Window } from './window/Window'
import { DOMParser } from './dom/DOMParser'
import { createAdapter } from './adapter'

// nodejs
if ('process' in globalThis) {
  import('worker_threads').then(w => w.parentPort?.once('message', main))
} else {
  self.addEventListener('message', ev => main(ev.data), { once: true })
}

async function main({ windowId, url }) {
  console.log('worker init', windowId, url)

  // unfortunately, we need native in worker too - there are many blocking APIs
  // and those would be impossible to emulate with parent<->worker postMessage()
  let nativeApi = await loadNativeApi()

  // get html
  const html = await nativeApi.readURL(url)

  // create document
  const docId = nativeApi.document_new()
  console.log('docId', docId)
  const document: any = new DOMParser(createAdapter(nativeApi, docId, url)).parseFromString(html, 'text/html')
  document.URL = url

  // create window
  const w = new Window(document)
  Object.setPrototypeOf(globalThis, w)

  // remove `location` from WorkerGlobalScope (TODO: navigator should go too)
  // @ts-expect-error
  delete globalThis.location
}
