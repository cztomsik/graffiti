import { isDeno } from '../util'

export const CANCEL_BUBBLE_IMMEDIATELY = Symbol()

// note that we are using provided Event if possible (deno)
class Event implements globalThis.Event {
  readonly type: string
  readonly timeStamp = Date.now()
  readonly bubbles: boolean
  readonly cancelable: boolean
  readonly composed: boolean
  isTrusted: boolean = true

  // TODO: phasing
  eventPhase = 0
  cancelBubble = false
  defaultPrevented = false

  target: EventTarget | null = null
  currentTarget: EventTarget | null = null

  constructor(type: string, eventInit?: EventInit) {
    this.type = type
    this.bubbles = eventInit?.bubbles ?? false
    this.cancelable = eventInit?.cancelable ?? false
    this.composed = eventInit?.composed ?? false
  }

  // deprecated but needed for WPT & react (during init)
  initEvent(type: string, bubbles = false, cancelable = false) {
    // this hack works for deno too (symbols are enumerable by default)
    const data = Object.assign({}, new Event(type, { bubbles, cancelable }))
    // @ts-expect-error
    delete data.isTrusted

    Object.assign(this, data)
  }

  preventDefault() {
    if (this.cancelable) {
      this.defaultPrevented = true
    }
  }

  stopPropagation() {
    this.cancelBubble = true
  }

  stopImmediatePropagation() {
    this[CANCEL_BUBBLE_IMMEDIATELY] = true
    this.stopPropagation()
  }

  get srcElement() {
    return this.target
  }

  get returnValue() {
    return !this.cancelBubble
  }

  set returnValue(v) {
    if (v === false) {
      this.preventDefault()
    }
  }

  static readonly NONE = 0
  static readonly CAPTURING_PHASE = 1
  static readonly AT_TARGET = 2
  static readonly BUBBLING_PHASE = 3

  // later
  composedPath

  NONE = Event.NONE
  CAPTURING_PHASE = Event.CAPTURING_PHASE
  AT_TARGET = Event.AT_TARGET
  BUBBLING_PHASE = Event.BUBBLING_PHASE
}

const ExportedEvent: typeof Event = isDeno ?globalThis.Event as any :Event
export { ExportedEvent as Event }

// monkey-patch for deno
if (!ExportedEvent.prototype.initEvent) {
  ExportedEvent.prototype.initEvent = Event.prototype.initEvent
}
