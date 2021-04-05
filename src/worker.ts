// internal (this is the "main script" of each window)
//
// note workers make things MORE COMPLICATED but:
// - we need separate require-chains becase some JS libraries depend on globals (and keep them)
//   - Deno doesn't have runInContext and it might not be enough anyway)
// - we want location.reload() for development purposes (live-reload, HMR)

import { native, loadNativeApi } from './native'
import { Window, makeGlobal } from './window/Window'
import { readURL, TODO, UNSUPPORTED } from './util'
import { getDocId } from './dom/Document'
import { parseIntoDocument } from './dom/DOMParser'

// nodejs
if ('process' in globalThis) {
  const { parentPort } = await import('worker_threads')
  globalThis.postMessage = (msg, _) => parentPort?.postMessage(msg)
  parentPort?.on('message', handleMessage)
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

  // setup env
  const { window, document } = new Window()
  makeGlobal(window)

  // init viewport
  const viewportId = native.viewport_new(800, 600, getDocId(document))
  VIEWPORT_REGISTRY.register(window, viewportId)

  // load html
  parseIntoDocument(document, await readURL(url))
  Object.assign(document, { defaultView: window, URL: url })

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

    // TODO: move all native methods to one big glue hash?
    native.viewport_render(windowId, viewportId)

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

const VIEWPORT_REGISTRY = new FinalizationRegistry(id => native.viewport_drop(id))
