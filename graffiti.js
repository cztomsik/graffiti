import { createRequire } from 'module'
const require = createRequire(import.meta.url)
const native = require('./zig-out/lib/graffiti.node')

const LISTENERS = Symbol()
const listeners = new WeakMap()

class EventTarget {
  get [LISTENERS]() {
    return listeners.get(this) ?? (listeners.set(this, {}), this[LISTENERS])
  }

  addEventListener(type, listener) {
    this[LISTENERS][type] = [...(this[LISTENERS][type] ?? []), listener]
  }

  dispatchEvent(event) {
    let curr = (event.target = this)
    do {
      event.currentTarget = curr
      for (const l of curr[LISTENERS][event.type] ?? []) {
        l.call(curr, event)
      }
    } while (!event.cancelBubble && (curr = curr.parentNode))
  }
}

// https://github.com/preactjs/preact/blob/0f8c55c6cfad8d5cc3aec6785c1f6940998b4782/src/diff/props.js#L102
Object.setPrototypeOf(
  EventTarget.prototype,
  new Proxy(Object.getPrototypeOf(EventTarget.prototype), {
    has: (_, p) => typeof p === 'string' && p.match(/on\w+/),
  })
)

// TODO: decide what to restore from https://github.com/cztomsik/graffiti/blob/50affb8419ff06a809099a85511042c08b0d1066/src/dom/Node.ts
class Node extends EventTarget {
  get parentNode() {
    return native.Node_parentNode(this)
  }

  get firstChild() {
    return native.Node_firstChild(this)
  }

  get previousSibling() {
    return native.Node_previousSibling(this)
  }

  get nextSibling() {
    return native.Node_nextSibling(this)
  }

  appendChild(child) {
    return native.Node_appendChild(this, child), child
  }

  insertBefore(child, before) {
    return before ? (native.Node_insertBefore(this, child, before), child) : this.appendChild(child)
  }

  replaceChild(child, oldChild) {
    return this.insertBefore(child, oldChild), this.removeChild(child)
  }

  removeChild(child) {
    return native.Node_removeChild(this, child), child
  }
}

class Element extends Node {
  get style() {
    // virtual object, stateless
    return new CSSStyleDeclaration(this)
  }

  setAttribute() {}
  addEventListener() {}
}

class Text extends Node {}

class Document extends Node {
  constructor() {
    return wrap(native.Document_init(), Document)
  }

  createElement(localName) {
    return wrap(native.Document_createElement(this, localName), Element)
  }

  createTextNode(data) {
    return wrap(native.Document_createTextNode(this, '' + data), Text)
  }

  elementFromPoint(x, y) {
    return native.Document_elementFromPoint(document, x, y)
  }
}

const wrap = (obj, Clz) => (Object.setPrototypeOf(obj, Clz.prototype), obj)

class CSSStyleDeclaration {
  #element

  constructor(element) {
    this.#element = element
  }

  setProperty(prop, value) {
    native.Element_setStyleProp(this.#element, prop, '' + value)
  }

  set cssText(v) {
    native.Element_setStyle(this.#element, v)
  }
}

Object.setPrototypeOf(
  CSSStyleDeclaration.prototype,
  new Proxy(Object.getPrototypeOf(CSSStyleDeclaration.prototype), {
    set: (_, k, v, style) => (style.setProperty(k, v), true),
  })
)

global.document = wrap(native.document, Document)
document.body = document.createElement('body')

const runLoop = async () => {
  while (await native.tick(null, ev => document.elementFromPoint(ev.x, ev.y).dispatchEvent(wrap(ev, Event)))) {}
}

Promise.resolve().then(runLoop)

class Event {
  get type() {
    const types = ['mousemove', 'scroll', 'mousedown', 'mouseup', 'keydown', 'keypress', 'key_up']
    return types[this.kind]
  }

  stopPropagation() {
    this.cancelBubble = true
  }

  preventDefault() {
    this.defaultPrevented = true
  }
}
