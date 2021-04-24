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
  width

  // deprecated but nice to have
  get bgColor() {
    return this.getAttribute('bgcolor') ?? ''
  }

  set bgColor(v) {
    this.setAttribute('bgcolor', v)
  }

  setAttribute(att, value) {
    super.setAttribute(att, value)

    if (att === 'bgcolor') {
      this.style.setProperty('background-color', '' + value)
    }

    // TODO: removeAttribute
  }
}
