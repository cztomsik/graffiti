import { UIEvent } from './index'

export class KeyboardEvent extends UIEvent implements globalThis.KeyboardEvent {
  altKey: boolean
  charCode: number
  ctrlKey: boolean
  isComposing: boolean
  key: string
  keyCode: number
  location: number
  metaKey: boolean
  repeat: boolean
  shiftKey: boolean

  DOM_KEY_LOCATION_STANDARD = 0
  DOM_KEY_LOCATION_LEFT = 1
  DOM_KEY_LOCATION_RIGHT = 2
  DOM_KEY_LOCATION_NUMPAD = 3

  constructor(type: string, eventInit?: KeyboardEventInit) {
    super(type, eventInit)
    this.altKey = eventInit?.altKey ?? false
    this.charCode = eventInit?.charCode ?? 0
    this.ctrlKey = eventInit?.ctrlKey ?? false
    this.isComposing = eventInit?.isComposing ?? false
    this.key = eventInit?.key ?? ''
    this.keyCode = eventInit?.keyCode ?? 0
    this.location = eventInit?.location ?? this.DOM_KEY_LOCATION_STANDARD
    this.metaKey = eventInit?.metaKey ?? false
    this.repeat = eventInit?.repeat ?? false
    this.shiftKey = eventInit?.shiftKey ?? false
  }

  initKeyboardEvent(type, bubbles, cancelable, view, key, location, ctrlKey, altKey, shiftKey, metaKey): void {
    this.initUIEvent(type, bubbles, cancelable, view, 0)
    this.key = key
    this.location = location
    this.ctrlKey = ctrlKey
    this.altKey = altKey
    this.shiftKey = shiftKey
    this.metaKey = metaKey
  }

  get char() {
    return String.fromCharCode(this.charCode)
  }

  get code() {
    return KEY_CODES[this.keyCode] ?? ''
  }

  getModifierState(keyArg: string): boolean {
    throw new Error('Method not implemented.')
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
