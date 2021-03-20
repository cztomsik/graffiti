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
import { TODO, UNSUPPORTED } from './util'

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

async function main({ windowId, url }) {
  console.log('worker init', windowId, url)

  // cleanup first (deno)
  for (const k of ['location']) {
    delete globalThis[k]
  }

  if (!globalThis.fetch) {
    // @ts-expect-error
    const { default: fetch } = await import('node-fetch')
    globalThis.fetch = fetch
  }
  
  // unfortunately, we need native in worker too - there are many blocking APIs
  // and those would be impossible to emulate with parent<->worker postMessage()
  await loadNativeApi()

  // create document
  const html = await readURL(url)
  const document: any = new DOMParser().parseFromString(html, 'text/html')
  document.URL = url

  // create window
  const w = new Window(document)

  // setup env
  Object.setPrototypeOf(globalThis, w)
  Object.assign(w, nodes)

  try {
    for (const { src, text } of document.querySelectorAll('script')) {
      if (src) {
        await import('' + new URL(src, url))
      } else {
        console.log('[eval]', text)
        const AsyncFunction = Object.getPrototypeOf(async function () {}).constructor
        new AsyncFunction('__filename', text.replace(/import\s+(".*?")/gi, 'await import(new URL($1, __filename))'))(
          url
        )
      }
    }
  } catch (e) {
    console.log(e)
    throw e
  }
}


async function readURL(url) {
  url = new URL(url)

  if (url.protocol === 'data:') {
    return TODO()
  }

  if (url.protocol === 'file:') {
    let fs = await import('fs/promises')
    return fs.readFile(url.pathname, 'utf-8')
  }

  if (url.protocol.match(/^https?:$/)) {
    return fetch(url).then(res => res.text())
  }

  return UNSUPPORTED()
}
