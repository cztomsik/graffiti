// TODO: https://github.com/cztomsik/graffiti/blob/bd1dfe61d3d7b5bfbf9184ecfb9e068dda982a60/src/events/EventTarget.ts

const LISTENERS = Symbol()
const listeners = new WeakMap()

export class EventTarget {
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
