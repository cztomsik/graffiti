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

// event state
let mousePos = [0, 0]
let overElement
let clickedElement

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
  // TODO: we can avoid namespacing with MessageChannel sent with first init
  //       (this is currently blocked by deno which does not yet support transferables)
  postMessage({ type: '__GFT', result, error }, '')
  native.wake_up()
}

async function main({ windowId, width, height, url, options }) {
  // unfortunately, we need native in worker too - there are many blocking APIs
  // and those would be impossible to emulate with parent<->worker postMessage()
  await loadNativeApi()

  // setup env
  const { window, document } = new Window()
  document.URL = url
  makeGlobal(window)

  // init viewport
  const viewportId = native.viewport_new(width, height, getDocId(document))
  document['__VIEWPORT_ID'] = viewportId
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
    // dispatch all events for this round
    let ev
    // TODO: windowId or viewportId? or something else?

    // TODO: async...
    while ((ev = native.window_next_event(windowId))) {
      handleEvent(ev)
    }

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

  // TODO: review, this is old code
  function handleEvent(event: [string, [number, number] | null, number | null]) {
    const [kind, vec2, u32] = event

    // TODO: only for mouse events
    let target = document.elementFromPoint(mousePos[0], mousePos[1]) ?? document.documentElement

    switch (kind) {
      case 'mousemove': {
        mousePos = vec2!

        const prevTarget = overElement
        overElement = target

        target._fire(kind)

        if (target !== prevTarget) {
          if (prevTarget) {
            prevTarget._fire('mouseout')
          }

          target._fire('mouseover')
        }

        return
      }

      case 'mousedown': {
        clickedElement = target
        return target._fire(kind)
      }

      case 'mouseup': {
        target._fire(kind)

        // TODO: only els with tabindex should be focusable

        // clicked & released at the same element
        if (target === clickedElement) {
          // focus change?
          if (target !== document.activeElement) {
            target.focus()
          }

          target._fire('click', { button: 0 })
        }

        return
      }

      // keydown - char is yet not known, keyCode maps to physical os-dependent key, repeats
      // keypress - char is known, repeats
      // keyup - key is up, after action, can be prevented
      // beforeinput - event.data contains new chars, may be empty when removing
      // input - like input, but after update (not sure if it's possible to do this on this level)
      case 'keydown': {
        const target = document.activeElement || document.documentElement
        const which = u32!
        const code = KEY_CODES[which]
        target._fire(kind, { which, keyCode: which, code })
        return
      }

      case 'keypress': {
        const target = document.activeElement || document.documentElement
        const charCode = u32!
        const key = String.fromCharCode(charCode)

        target._fire(kind, { charCode, key })
        return
      }

      case 'resize': {
        native.viewport_resize(viewportId, vec2![0], vec2![1])
        return
      }

      case 'close': {
        console.log('TODO: close worker somehow (or tell main process to do it)')
        return
      }
    }
  }

  // TODO: more mapping
  // https://w3c.github.io/uievents-code/#keyboard-key-codes
  // https://keycode.info/
  //
  // BTW: it should be fast because it's just array lookup but I'm not sure if it's not holey
  const KEY_CODES = Object.assign(Array(40).fill(''), {
    8: 'Backspace',
    9: 'Tab',
    13: 'Enter',
    27: 'Escape',
    32: 'Space',
    37: 'ArrowLeft',
    38: 'ArrowUp',
    39: 'ArrowRight',
    40: 'ArrowDown',

    // TODO: more codes
  })
}

const VIEWPORT_REGISTRY = new FinalizationRegistry(id => native.viewport_drop(id))
