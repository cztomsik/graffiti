// internal (this is the "main script" of each window)
//
// note workers make things MORE COMPLICATED but:
// - we need separate require-chains becase some JS libraries depend on globals (and keep them)
//   - Deno doesn't have runInContext and it might not be enough anyway)
// - we want location.reload() for development purposes (live-reload, HMR)

import { native, loadNativeApi } from './native'
import { Window } from './window/Window'
import { DOMParser } from './dom/DOMParser'
import * as nodes from './nodes/index'
import * as events from './events/index'
import { readURL, TODO, UNSUPPORTED } from './util'
import { getDocId } from './nodes/Document'

// nodejs
if ('process' in globalThis) {
  import('worker_threads').then(w => {
    globalThis.postMessage = (msg, _) => w.parentPort?.postMessage(msg)
    w.parentPort?.on('message', handleMessage)
  })
} else {
  self.addEventListener('message', ev => handleMessage(ev.data))
}

async function handleMessage(msg) {
  try {
    switch (msg.type) {
      case 'init':
        return send(await main(msg))
      case 'eval':
        return send(eval.call(null, msg.js))
    }
  } catch (e) {
    send(undefined, `${e}\n${e.stack}`)
  }
}

async function send(result, error?) {
  postMessage({ type: '__GFT', result, error }, '')
  native.wake_up()
}

async function main({ windowId, url, options }) {
  // unfortunately, we need native in worker too - there are many blocking APIs
  // and those would be impossible to emulate with parent<->worker postMessage()
  await loadNativeApi()

  // cleanup first (deno)
  for (const k of [/*'Event', 'EventTarget',*/ 'location']) {
    delete globalThis[k]
  }

  if (!globalThis.fetch) {
    // @ts-expect-error
    const { default: fetch } = await import('node-fetch')
    globalThis.fetch = fetch
  }

  // create document
  const html = await readURL(url)
  const document: any = new DOMParser().parseFromString(html, 'text/html')
  document.URL = url

  // create window
  const window = new Window(document)

  // setup env
  Object.setPrototypeOf(globalThis, window)
  Object.assign(window, nodes)
  Object.assign(window, events)

  // start event handling & rendering
  loop()

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
      // WPT is relying on global `var`s
      if (options.evalSrc) {
        await evalScript(await readURL('' + new URL(src, url)))
      } else {
        await import('' + new URL(src, url))
      }
    } else {
      evalScript(text)
    }
  }

  window._fire('load')

  function loop() {
    // TODO: dispatch all events for this round

    native.render(windowId, getDocId(document))

    setTimeout(loop, 100)
  }

  async function evalScript(script) {
    const module = script.replace(/import\s+(".*?")/gi, 'await import(new URL($1, __filename))')

    // ESM
    if (module !== script) {
      const AsyncFunction = Object.getPrototypeOf(async function () {}).constructor
      return new AsyncFunction('__filename', module)(url)
    }

    // legacy, vars & functions are accumulated
    return eval.call(null, script)
  }
}
