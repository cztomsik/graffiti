// internal (this is the "main script" of each window)
//
// note workers make things MORE COMPLICATED but:
// - we need separate require-chains becase some JS libraries depend on globals (and keep them)
//   - Deno doesn't have runInContext and it might not be enough anyway)
// - we want location.reload() for development purposes (live-reload, HMR)

import { loadNativeApi } from './native'
import { Window } from './window/Window'
import { DOMParser } from './dom/DOMParser'
import * as nodes from './nodes/index'

// cleanup first (deno)
for (const k of ['location']) {
  delete globalThis[k]
}

// nodejs
if ('process' in globalThis) {
  import('worker_threads').then(w => w.parentPort?.on('message', handleMessage))
} else {
  self.addEventListener('message', ev => handleMessage(ev.data))
}

function handleMessage(msg) {
  switch (msg.type) {
    case 'init': return main(msg)
    // TODO: type, id
    case 'eval': return postMessage(eval(msg.js), '')
  }
}

// TODO: (pre)loader and/or html
async function main({ windowId, url }) {
  console.log('worker init', windowId, url)

  // unfortunately, we need native in worker too - there are many blocking APIs
  // and those would be impossible to emulate with parent<->worker postMessage()
  await loadNativeApi()

  // create document
  const document: any = new DOMParser().parseFromString('<html><head><title></title><style></style></head><body><div class="app todoapp" id="page"></div></body></html>', 'text/html')
  document.URL = 'graffiti:///'

  // create window
  const w = new Window(document)

  // setup env
  Object.setPrototypeOf(globalThis, w)
  Object.assign(w, nodes)

  try {
    await import(url)
  } catch (e) {
    console.log(e)
  }
}
