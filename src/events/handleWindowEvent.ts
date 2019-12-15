import { Document } from '../dom/Document'
import { Event } from './Event'
import { EventKind } from '../core/nativeApi'

// events
//
// there is so much work to be done I don't even know where to start but here
// are some useful notes:
// - reuse existing event types if possible (either RN or DOM), it doesn't have to be 100%
//   the same but it's great not having to re-learn everything
// - bubbling has its issues but any different approach would be very surprising

// event is [kind, target, key]
export function handleWindowEvent(document: Document, event) {
  // console.log(event)

  let e = event as any
  let target = (e[1] !== undefined) ?document._getEl(e[1]) :document.documentElement

  switch (event[0]) {
    case EventKind.Close: {
      return process.exit(0)
    }
    case EventKind.MouseMove: {
      const prevTarget = document._overElement
      document._overElement = target

      target._fire('mousemove')

      if (target !== prevTarget) {
        if (prevTarget) {
          prevTarget._fire('mouseout')
        }

        target._fire('mouseover')
      }

      return
    }
    case EventKind.MouseDown: {
      document._clickedElement = target
      return target._fire('mousedown')
    }
    case EventKind.MouseUp: {
      target._fire('mouseup')

      // TODO: only els with tabindex should be focusable

      // clicked & released at the same element
      if (target === document._clickedElement) {
        // focus change?
        if (target !== document.activeElement) {
          target.focus()
        }

        target._fire('click', { button: 0 })
      }

      return
    }

    // keydown - char is yet not known, scancode maps to physical os-dependent key, repeats
    // keypress - char is known, repeats
    // keydown - key is up, after action, can be prevented
    // beforeinput - event.data contains new chars, may be empty when removing
    // input - like input, but after update (not sure if it's possible to do this on this level)
    case EventKind.KeyDown: {
      const target = document.activeElement || document.documentElement
      const [which, code] = getKey(event[2])
      target._fire('keydown', { which, keyCode: which, code })
      return
    }
    case EventKind.KeyPress: {
      const target = document.activeElement || document.documentElement
      const charCode = event[2]
      const key = String.fromCharCode(charCode)

      target._fire('keypress', { charCode, key })
      return
    }
  }
}

// TODO: https://w3c.github.io/uievents-code/#keyboard-key-codes
// TODO: array lookup
function getKey(scancode) {
  // TODO: return (js-specific numbers) from native, scancodes are platform-specific
  switch (scancode) {
    case 49:
      return [32, 'Space']
    case 36:
      return [13, 'Enter']
    case 123:
      return [37, 'ArrowLeft']
    case 124:
      return [39, 'ArrowRight']
    case 125:
      return [40, 'ArrowDown']
    case 126:
      return [38, 'ArrowUp']
    case 51:
      return [8, 'Backspace']
    case 53:
      return [27, 'Escape']
    }

  return []
}
