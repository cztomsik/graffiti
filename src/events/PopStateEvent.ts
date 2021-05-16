import { Event } from './index'

export class PopStateEvent extends Event implements globalThis.PopStateEvent {
  state: any

  constructor(type: string, eventInit?: PopStateEventInit) {
    super(type, eventInit)
    this.state = eventInit?.state
  }
}
