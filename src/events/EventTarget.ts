import { UNSUPPORTED } from '../core/utils';

export class EventTarget implements globalThis.EventTarget {
  _listeners: { [type in string]?: readonly EventListenerOrEventListenerObject[] } = {}

  addEventListener(type, listener) {
    this._listeners[type] = [...this._getListeners(type), listener]
  }

  removeEventListener(type, listener) {
    this._listeners[type] = this._getListeners(type).filter(l => l !== listener)
  }

  dispatchEvent(event) {
    event.target = this

    this._dispatch(event)

    return !event.defaultPrevented
  }

  _dispatch(event) {
    event.currentTarget = this

    for (const l of this._getListeners(event.type)) {
      if ('handleEvent' in l) {
        l.handleEvent(event)
      } else {
        l.call(this, event)
      }

      if (event.cancelBubbleImmediately) {
        break
      }
    }

    if (!event.cancelBubble) {
      this._bubble(event)
    }
  }

  _bubble(event) {
    const parent = this._getTheParent()

    if (parent) {
      parent._dispatch(event)
    }
  }

  // https://dom.spec.whatwg.org/#get-the-parent
  _getTheParent() {
    return null
  }

  _getListeners(type) {
    return this._listeners[type] || []
  }
}

// preact does some golfing with casing: name = (nameLower in dom ? nameLower : name).slice(2);
// https://github.com/preactjs/preact/blob/013dc382cf7239422e834e74a6ab0b592c5a9c43/src/diff/props.js#L80
//
// this is also opportunity to disallow on* properties
//
// TODO: other event types
// BTW: just lower-casing type everywhere is not enough (tried already) but proxy in the prototype chain might work
for (const k of ['click']) {
  Object.defineProperty(EventTarget.prototype, `on${k}`, {
    set: v => {
      // throw unless no-op
      // (react-dom sets this to avoid some safari bug)
      if (v && v.toString() !== 'function noop () {}') {} else {
        UNSUPPORTED()
      }
    }
  })
}
