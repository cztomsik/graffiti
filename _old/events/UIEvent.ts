import { Event } from './index'

export class UIEvent extends Event implements globalThis.UIEvent {
  detail: number;
  view: Window | null;

  constructor(type: string, eventInit?: UIEventInit) {
    super(type, eventInit)

    this.detail = eventInit?.detail ?? 0
    this.view = eventInit?.view ?? null
  }

  get which() {
    return this['keyCode'] ?? 0
  }

  initUIEvent(type, bubbles, cancelable, view, detail) {
    this.initEvent(type, bubbles, cancelable)
    this.view = view
    this.detail = detail
  }
}
