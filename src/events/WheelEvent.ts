import { MouseEvent } from './index'

export class WheelEvent extends MouseEvent implements globalThis.WheelEvent {
  deltaMode: number
  deltaX: number
  deltaY: number
  deltaZ: number

  DOM_DELTA_PIXEL = 0
  DOM_DELTA_LINE = 1
  DOM_DELTA_PAGE = 2

  constructor(type: string, eventInit?: WheelEventInit) {
    super(type, eventInit)
    this.deltaMode = eventInit?.deltaMode ?? this.DOM_DELTA_PIXEL
    this.deltaX = eventInit?.deltaX ?? 0
    this.deltaY = eventInit?.deltaY ?? 0
    this.deltaZ = eventInit?.deltaZ ?? 0
  }
}
