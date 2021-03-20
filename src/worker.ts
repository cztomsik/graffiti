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

async function handleMessage(msg) {
  try {
    switch (msg.type) {
      case 'init':
        return postMessage({ type: '__GFT', result: await main(msg) }, '')
      case 'eval':
        return postMessage({ type: '__GFT', result: eval(msg.js) }, '')
    }
  } catch (e) {
    postMessage({ type: '__GFT', error: `${e.message}\n${e.stack}` }, '')
  }
}

async function main({ windowId, url }) {
  //console.log('worker init', windowId, url)

  // TODO: wpt global (find a way to pass some custom init or something)
  globalThis.rv = null

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
  const window = new Window(document)

  // setup env
  Object.setPrototypeOf(globalThis, window)
  Object.assign(window, nodes)

  // we replace <link> with <style> which works surprisingly well
  // TODO: qsa link[rel="stylesheet"]
  for (const link of document.querySelectorAll('link')) {
    if (link.rel === 'stylesheet' && link.href) {
      const style = document.createElement('style')
      style.textContent = await readURL('' + new URL(link.getAttribute('href'), url))
      link.replaceWith(style)
    }
  }

  // run (once) all the scripts
  for (const { src, text } of document.querySelectorAll('script')) {
    if (src) {
      //console.log('[script]', src)
      await import('' + new URL(src, url))
    } else {
      //console.log('[eval]', text)
      const AsyncFunction = Object.getPrototypeOf(async function () {}).constructor
      await new AsyncFunction(
        '__filename',
        text.replace(/import\s+(".*?")/gi, 'await import(new URL($1, __filename))')
      )(url)
    }
  }

  window._fire('load')
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
