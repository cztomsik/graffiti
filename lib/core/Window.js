// TODO: https://github.com/cztomsik/graffiti/blob/bd1dfe61d3d7b5bfbf9184ecfb9e068dda982a60/src/window/Window.ts

import { wrap } from './native.js'
import { EventTarget } from './EventTarget.js'

export class Window extends EventTarget {
  get window() {
    return this
  }

  get self() {
    return this
  }

  handleEvent(ev) {
    //console.log(ev)
    const el = document.elementFromPoint(ev.x, ev.y)
    el.dispatchEvent(wrap(Event, ev))
  }
}

class Event {
  kind

  get type() {
    const types = ['close', 'mousemove', 'scroll', 'mousedown', 'mouseup', 'click', 'keydown', 'keypress', 'keyup']
    return types[this.kind]
  }

  stopPropagation() {
    this.cancelBubble = true
  }

  preventDefault() {
    this.defaultPrevented = true
  }
}
