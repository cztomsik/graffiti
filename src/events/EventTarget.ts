export class EventTarget implements globalThis.EventTarget {
  listeners: { [type in string]?: readonly EventListenerOrEventListenerObject[] } = {}

  addEventListener(type, listener) {
    this.listeners[type] = [...this._getListeners(type), listener]
  }

  removeEventListener(type, listener) {
    this.listeners[type] = this._getListeners(type).filter(l => l !== listener)
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
    return this.listeners[type] || []
  }
}
