import { NOOP } from '../utils'
import { WindowEvent } from '../generated';

// events
//
// there is so much work to be done I don't even know where to start but here
// are some useful notes:
// - reuse existing event types if possible (either RN or DOM), it doesn't have to be 100%
//   the same but it's great not having to re-learn everything
// - bubbling has its issues but any different approach would be very surprising
// - it should be enough to support only one listener for each (surface, type) pair (vs addEventListener)
//   - simpler/faster, edge cases can be handled in user-space (if necessary at all)
// - View shouldn't be responsible for registering events, it doesn't even know about window
//   - and we want it to be stateless
export class NotSureWhat {
  listeners: EventListeners = {
    onFocus: [],
    onBlur: [],
    onKeyDown: [],
    onKeyUp: [],
    onKeyPress: [],
    onMouseMove: [],
    onMouseOver: [],
    onMouseOut: [],
    onMouseDown: [],
    onMouseUp: [],
    onClick: []
  }
  moveTarget = 0
  downTarget = 0
  // TODO: only els with tabindex should be focusable
  focusTarget = 0

  constructor(private parents) {
    // root
    this.alloc()
  }

  alloc() {
    for (const k in this.listeners) {
      this.listeners[k].push(NOOP)
    }
  }

  handleWindowEvent(event: WindowEvent) {
    console.log(event)

    switch (event.tag) {
      case 'Close': {
        return process.exit(0)
      }
      case 'MouseMove': {
        const prevTarget = this.moveTarget
        const target = this.moveTarget = event.value.target
        this.dispatch(this.listeners.onMouseMove, target, { target })

        if (target !== prevTarget) {
          this.dispatch(this.listeners.onMouseOut, prevTarget, { target: prevTarget })
          this.dispatch(this.listeners.onMouseOver, target, { target })
        }

        return
      }
      case 'MouseDown': {
        const target = this.downTarget = event.value.target
        return this.dispatch(this.listeners.onMouseDown, target, { target })
      }
      case 'MouseUp': {
        const target = event.value.target

        this.dispatch(this.listeners.onMouseUp, target, { target })

        if (target === this.downTarget) {
          if (target !== this.focusTarget) {
            this.dispatch(this.listeners.onBlur, this.focusTarget, { target: this.focusTarget })
            this.focusTarget = target
            this.dispatch(this.listeners.onFocus, target, { target })
          }

          this.dispatch(this.listeners.onClick, target, { target })
        }

        return
      }

      // keydown - char is yet not known, scancode maps to physical os-dependent key, repeats
      // keypress - char is known, repeats
      // keydown - key is up, after action, can be prevented
      // beforeinput - event.data contains new chars, may be empty when removing
      // input - like input, but after update (not sure if it's possible to do this on this level)
      case 'KeyDown': {
        const target  = this.focusTarget
        const code = getKeyCode(event.value)
        this.dispatch(this.listeners.onKeyDown, target, { target, code })
        return
      }
      case 'KeyPress': {
        const target  = this.focusTarget
        const key = String.fromCharCode(event.value)

        this.dispatch(this.listeners.onKeyPress, target, { target, key })
        return
      }
    }
  }

  setEventListener<K extends keyof EventMap>(id, type: K, listener: Listener<EventMap[K]>) {
    if (!(type in this.listeners)) {
      throw new Error(`${type} is not supported`)
    }

    this.listeners[type][id] = listener
  }

  // dispatch event to target and all its parents
  // TODO: stopPropagation()
  dispatch<T>(listeners: Listener<T>[], id, event) {
    while (id) {
      listeners[id](event)

      id = this.parents[id]
    }
  }
}

// TODO: https://w3c.github.io/uievents-code/#keyboard-key-codes
function getKeyCode(scancode) {
  switch (scancode) {
    case 36: return 'Enter'
    case 51: return 'Backspace'
  }
}

// events we support
interface EventMap {
  onFocus: FocusEvent
  onBlur: FocusEvent
  onKeyDown: KeyboardEvent
  onKeyUp: KeyboardEvent
  onKeyPress: KeyboardEvent
  onMouseMove: MouseEvent,
  onMouseOver: MouseEvent,
  onMouseOut: MouseEvent,
  onMouseDown: MouseEvent,
  onMouseUp: MouseEvent,
  onClick: MouseEvent
}

type Listener<E> = (ev: E) => any

// struct of arrays (listeners for each type)
type EventListeners = {
  [K in keyof EventMap]: Listener<EventMap[K]>[]
}
