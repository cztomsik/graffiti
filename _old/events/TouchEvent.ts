import { UIEvent } from './index'

export class TouchEvent extends UIEvent implements globalThis.TouchEvent {
  altKey: boolean
  ctrlKey: boolean
  metaKey: boolean
  shiftKey: boolean

  // TODO
  touches: TouchList
  targetTouches: TouchList
  changedTouches: TouchList

  constructor(type: string, eventInit?: TouchEventInit) {
    super(type, eventInit)
    this.altKey = eventInit?.altKey ?? false
    this.ctrlKey = eventInit?.ctrlKey ?? false
    this.metaKey = eventInit?.metaKey ?? false
    this.shiftKey = eventInit?.shiftKey ?? false

    // TODO
    this.touches = eventInit?.touches as any
    this.targetTouches = eventInit?.targetTouches as any
    this.changedTouches = eventInit?.changedTouches as any
  }
}
