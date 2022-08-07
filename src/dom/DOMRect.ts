// TODO: DOMRectReadOnly + make this mutable

import { IDOMRect } from '../types'

export class DOMRect implements IDOMRect {
  constructor(public x = 0, public y = 0, public width = 0, public height = 0) {}

  get left() {
    return this.width < 0 ? this.x + this.width : this.x
  }

  get right() {
    return this.width >= 0 ? this.x + this.width : this.x
  }

  get top() {
    return this.height < 0 ? this.y + this.height : this.y
  }

  get bottom() {
    return this.height >= 0 ? this.y + this.height : this.y
  }

  toJSON() {
    const { x, y, width, height, left, top, right, bottom } = this
    return { x, y, width, height, left, top, right, bottom }
  }
}
