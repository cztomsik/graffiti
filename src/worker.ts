// internal (this is the "main script" of each window)
//
// note workers make things MORE COMPLICATED but:
// - we need separate require-chains becase some JS libraries depend on globals (and keep them)
//   - Deno doesn't have runInContext and it might not be enough anyway)
// - we want location.reload() for development purposes (live-reload, HMR)

import { native, loadNativeApi, register, getNativeId } from './native'
import { Window, makeGlobal } from './window/Window'
import { LITTLE_ENDIAN, readURL } from './util'
import { parseIntoDocument } from './dom/DOMParser'
import { loadStyles } from './dom/HTMLLinkElement'
import { runScripts } from './dom/HTMLScriptElement'

// event state
const eventBytes = new Uint8Array(24)
const eventView = new DataView(eventBytes.buffer)
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
  postMessage({ type: '__GFT', result, error })
  native.gft_App_wake_up()
}

async function main({ windowId, width, height, url, options }) {
  // unfortunately, we need native in worker too - there are many blocking APIs
  // and those would be impossible to emulate with parent<->worker postMessage()
  await loadNativeApi()

  // setup env
  const { window, document } = new Window()
  document.URL = url
  makeGlobal(window)

  // init renderer
  const renderer = native.gft_Renderer_new(getNativeId(document), width, height)
  register(window, renderer)

  // load html
  parseIntoDocument(document, await readURL(url))
  Object.assign(document, { defaultView: window, URL: url })

  // start event handling & rendering
  loop()

  // load (once) remote styles
  await loadStyles()

  // run (once) all the scripts
  await runScripts()

  window.dispatchEvent(new Event('load'))

  function loop() {
    // dispatch all events for this round
    // TODO: windowId or viewportId? or something else?

    // TODO: async...
    while (native.gft_Window_next_event(windowId, eventBytes)) {
      handleEvent()
    }

    native.gft_Renderer_render(renderer)

    setTimeout(loop, 100)
  }

  // TODO: review, this is old code
  function handleEvent() {
    // TODO: codegen
    enum EventKind {
      CursorPos = 0,
      MouseDown = 1,
      MouseUp = 2,
      Scroll = 3,
  
      // JS e.which
      KeyUp = 4,
      KeyDown = 5,
      KeyPress = 6,
  
      Resize = 7,
      FramebufferSize = 8,
      Close = 9,
    }

    // TODO: only for mouse events
    let target = document.documentElement //document.elementFromPoint(mousePos[0], mousePos[1]) ?? document.documentElement

    switch (eventView.getUint32(0, LITTLE_ENDIAN)) {
      case EventKind.CursorPos: {
        mousePos = [eventView.getFloat32(4), eventView.getFloat32(8)]

        const prevTarget = overElement
        overElement = target

        target.dispatchEvent(new MouseEvent('mousemove', { bubbles: true, cancelable: true }))

        if (target !== prevTarget) {
          if (prevTarget) {
            prevTarget.dispatchEvent(new MouseEvent('mouseout', { bubbles: true, cancelable: true }))
          }

          target.dispatchEvent(new MouseEvent('mouseover', { bubbles: true, cancelable: true }))
        }

        return
      }

      case EventKind.MouseDown: {
        clickedElement = target
        target.dispatchEvent(new MouseEvent('mousedown', { bubbles: true, cancelable: true }))

        return
      }

      case EventKind.MouseUp: {
        target.dispatchEvent(new MouseEvent('mouseup', { bubbles: true, cancelable: true }))

        // TODO: only els with tabindex should be focusable

        // clicked & released at the same element
        if (target === clickedElement) {
          // focus change?
          if (target !== document.activeElement) {
            target.focus()
          }

          target.dispatchEvent(new MouseEvent('click', { bubbles: true, cancelable: true, button: 0 }))
        }

        return
      }

      // keydown - char is yet not known, keyCode maps to physical os-dependent key, repeats
      // keypress - char is known, repeats
      // keyup - key is up, after action, can be prevented
      // beforeinput - event.data contains new chars, may be empty when removing
      // input - like input, but after update (not sure if it's possible to do this on this level)
      case EventKind.KeyDown: {
        const target = document.activeElement || document.documentElement
        const keyCode = eventView.getUint32(4)
        target.dispatchEvent(new KeyboardEvent('keydown', { bubbles: true, cancelable: true, keyCode }))
        return
      }

      case EventKind.KeyPress: {
        const target = document.activeElement || document.documentElement
        const charCode = eventView.getUint32(4)
        const key = String.fromCharCode(charCode)

        target.dispatchEvent(new KeyboardEvent('keypress', { bubbles: true, cancelable: true, charCode, key }))
        return
      }

      case EventKind.Resize: {
        native.gft_Renderer_resize(
          renderer,
          eventView.getFloat32(4, LITTLE_ENDIAN),
          eventView.getFloat32(8, LITTLE_ENDIAN)
        )
        return
      }

      case EventKind.Close: {
        console.log('TODO: close worker somehow (or tell main process to do it)')
        return
      }
    }
  }
}
