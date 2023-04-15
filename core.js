// minimal subset of DOM and CSSOM to get Preact working
// everything else is supposed to be in polyfills
//
// note that everything is in one file, this is intentional, we want to keep
// things simple & minimal

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
      // @ts-ignore
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
    return native.Node_parent_node(this)
  }

  get firstChild() {
    return native.Node_first_child(this)
  }

  get previousSibling() {
    return native.Node_previous_sibling(this)
  }

  get nextSibling() {
    return native.Node_next_sibling(this)
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
  _style = null

  get style() {
    return this._style || (this._style = wrap(native.Element_style(this), CSSStyleDeclaration))
  }

  getAttribute(name) {
    return native.Element_getAttribute(this, name)
  }

  setAttribute(name, value) {
    native.Element_setAttribute(this, name, '' + value)
  }

  removeAttribute(name) {
    native.Element_removeAttribute(this, name)
  }
}

class CharacterData extends Node {
  get data() {
    return native.CharacterData_data(this)
  }

  set data(data) {
    native.CharacterData_setData(this, '' + data)
  }
}

class Text extends CharacterData {}
class Comment extends CharacterData {}

class Document extends Node {
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
  get length() {
    return native.CSSStyleDeclaration_length(this)
  }

  item(i) {
    return native.CSSStyleDeclaration_item(this, i)
  }

  getPropertyValue(prop) {
    return native.CSSStyleDeclaration_getPropertyValue(this, prop)
  }

  setProperty(prop, value) {
    native.CSSStyleDeclaration_setProperty(this, prop, '' + value)
  }

  removeProperty(prop) {
    native.CSSStyleDeclaration_removeProperty(this, prop)
  }

  get cssText() {
    return native.CSSStyleDeclaration_cssText(this)
  }

  set cssText(cssText) {
    native.CSSStyleDeclaration_setCssText(this, cssText)
  }
}

// TODO(perf): try to get list of properties from native and define getters/setters
Object.setPrototypeOf(
  CSSStyleDeclaration.prototype,
  new Proxy(Object.getPrototypeOf(CSSStyleDeclaration.prototype), {
    get: (_, k) => native.CSSStyleDeclaration_getProperty(this, k),
    set: (_, k, v, style) => (style.setProperty(k, v), true),
  })
)

// TODO: weak-ref GC (*anyopaque in event creates ref to temporal object which is collected if not used)
// (can be trickier because of unknown order of finalization + possible re-creation later/before)

class Window extends EventTarget {
  handleEvent(ev) {
    //console.log(ev)
    const el = document.elementFromPoint(ev.x, ev.y)
    el.dispatchEvent(wrap(ev, Event))
  }
}

// TODO: just pass protos to init()
// TODO: it could also patch globals
Object.assign(global, native.init())
wrap(document, Document)
document.body = document.createElement('body')
document.appendChild(document.body)
wrap(window, Window)
// TODO: should be window
document.body.addEventListener('close', () => process.exit())

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
