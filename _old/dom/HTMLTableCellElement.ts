import { HTMLElement } from './index'

export class HTMLTableCellElement extends HTMLElement implements globalThis.HTMLTableCellElement {
  abbr
  align
  axis
  cellIndex
  ch
  chOff
  colSpan
  headers
  height
  noWrap
  rowSpan
  scope
  vAlign

  // deprecated but nice to have
  get bgColor() {
    return this.getAttribute('bgcolor') ?? ''
  }

  set bgColor(v) {
    this.setAttribute('bgcolor', v)
  }

  get width() {
    return this.getAttribute('width') ?? ''
  }

  set width(v) {
    this.setAttribute('width', v)
  }

  setAttribute(att, value) {
    super.setAttribute(att, value)

    if (att === 'bgcolor') {
      this.style.setProperty('background-color', '' + value)
    }

    if (att === 'width') {
      const v = '' + value
      this.style.setProperty('width', v.match(/^\d+$/) ? v + 'px' : v)
    }

    // TODO: removeAttribute
  }
}
