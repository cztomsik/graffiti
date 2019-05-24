export class Event {
  cancelBubble = false
  cancelBubbleImmediately = false
  defaultPrevented = false

  constructor(public type) {}

  preventDefault() {
    this.defaultPrevented = true
  }

  stopPropagation() {
    this.cancelBubble = true
  }

  stopImmediatePropagation() {
    this.cancelBubbleImmediately = true
    this.stopPropagation()
  }
}
