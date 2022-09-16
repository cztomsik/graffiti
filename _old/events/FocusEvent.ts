import { UIEvent } from './index'

export class FocusEvent extends UIEvent implements globalThis.FocusEvent {
  relatedTarget: EventTarget | null

  constructor(type: string, eventInit?: FocusEventInit) {
    super(type, eventInit)
    this.relatedTarget = eventInit?.relatedTarget ?? null
  }
}
