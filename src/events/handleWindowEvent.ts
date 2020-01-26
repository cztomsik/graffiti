import { Document } from '../dom/Document'
import { Event } from './Event'
import { Event as SceneEvent } from '../core/nativeApi'

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
    case SceneEvent.TAGS.Close: {
      return process.exit(0)
    }
    case SceneEvent.TAGS.MouseMove: {
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
    case SceneEvent.TAGS.MouseDown: {
      document._clickedElement = target
      return target._fire('mousedown')
    }
    case SceneEvent.TAGS.MouseUp: {
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

    // keydown - char is yet not known, keyCode maps to physical os-dependent key, repeats
    // keypress - char is known, repeats
    // keydown - key is up, after action, can be prevented
    // beforeinput - event.data contains new chars, may be empty when removing
    // input - like input, but after update (not sure if it's possible to do this on this level)
    case SceneEvent.TAGS.KeyDown: {
      const target = document.activeElement || document.documentElement
      const which = event[2]
      const code = KEY_CODES[which]
      target._fire('keydown', { which, keyCode: which, code })
      return
    }
    case SceneEvent.TAGS.KeyPress: {
      const target = document.activeElement || document.documentElement
      const charCode = event[2]
      const key = String.fromCharCode(charCode)

      target._fire('keypress', { charCode, key })
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
