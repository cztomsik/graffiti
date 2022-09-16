import { UIEvent } from './index'

export class MouseEvent extends UIEvent implements globalThis.MouseEvent {
  altKey: boolean
  button: number
  buttons: number
  clientX: number
  clientY: number
  ctrlKey: boolean
  metaKey: boolean
  movementX: number
  movementY: number
  relatedTarget: EventTarget | null
  screenX: number
  screenY: number
  shiftKey: boolean

  // TODO
  offsetX
  offsetY
  pageX
  pageY

  constructor(type: string, eventInit?: MouseEventInit) {
    super(type, eventInit)
    this.altKey = eventInit?.altKey ?? false
    this.button = eventInit?.button ?? 0
    this.buttons = eventInit?.buttons ?? 0
    this.clientX = eventInit?.clientX ?? 0
    this.clientY = eventInit?.clientY ?? 0
    this.ctrlKey = eventInit?.ctrlKey ?? false
    this.metaKey = eventInit?.metaKey ?? false
    this.movementX = eventInit?.movementX ?? 0
    this.movementY = eventInit?.movementY ?? 0
    this.relatedTarget = eventInit?.relatedTarget ?? null
    this.screenX = eventInit?.screenX ?? 0
    this.screenY = eventInit?.screenY ?? 0
    this.shiftKey = eventInit?.shiftKey ?? false
  }

  get x() {
    return this.clientX
  }

  get y() {
    return this.clientY
  }

  getModifierState(keyArg: string): boolean {
    throw new Error('Method not implemented.')
  }

  initMouseEvent(
    typeArg,
    bubbles,
    cancelable,
    view,
    detail,
    screenX,
    screenY,
    clientX,
    clientY,
    ctrlKey,
    altKey,
    shiftKey,
    metaKey,
    button,
    relatedTarget
  ) {
    throw new Error('Method not implemented.')
  }
}
