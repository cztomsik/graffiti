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
  let target = (e[1] !== undefined) && document._getEl(e[1])

  switch (event[0]) {
    case EventKind.Close: {
      return process.exit(0)
    }
    case EventKind.MouseMove: {
      const prevTarget = document._overElement
      dispatch('mousemove', document._overElement = target, { target })

      if (target !== prevTarget) {
        dispatch('mouseout', prevTarget, { target: prevTarget })
        dispatch('mouseover', target, { target })
      }

      return
    }
    case EventKind.MouseDown: {
      return dispatch('mousedown', document._clickedElement = target, { target })
    }
    case EventKind.MouseUp: {
      dispatch('mouseup', target, { target })

      // TODO: only els with tabindex should be focusable
      if (target === document._clickedElement) {
        if (target !== document.activeElement) {
          dispatch('blur', document.activeElement, {
            target: document.activeElement
          })

          dispatch('focus', document.activeElement = target, { target })
        }

        dispatch('click', target, { target, button: 0 })
      }

      return
    }

    // keydown - char is yet not known, scancode maps to physical os-dependent key, repeats
    // keypress - char is known, repeats
    // keydown - key is up, after action, can be prevented
    // beforeinput - event.data contains new chars, may be empty when removing
    // input - like input, but after update (not sure if it's possible to do this on this level)
    case EventKind.KeyDown: {
      const target = document.activeElement
      const code = getKeyCode(event[2])
      dispatch('keydown', target, { target, code })
      return
    }
    case EventKind.KeyPress: {
      const target = document.activeElement
      const key = String.fromCharCode(event[2])

      dispatch('keypress', target, { target, key })
      return
    }
  }

  function dispatch(type, el = document as any, data) {
    const e = Object.assign(new Event(type), data)
    el.dispatchEvent(e)
  }
}

// TODO: https://w3c.github.io/uievents-code/#keyboard-key-codes
function getKeyCode(scancode) {
  switch (scancode) {
    case 36:
      return 'Enter'
    case 51:
      return 'Backspace'
  }
}
