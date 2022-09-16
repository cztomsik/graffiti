import { Event } from './index'

export class HashChangeEvent extends Event implements globalThis.HashChangeEvent {
  newURL: string
  oldURL: string

  constructor(type: string, eventInit?: HashChangeEventInit) {
    super(type, eventInit)
    this.newURL = eventInit?.newURL ?? ''
    this.oldURL = eventInit?.oldURL ?? ''
  }
}
