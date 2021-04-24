import { HTMLElement } from './index'

export class HTMLTableElement extends HTMLElement implements globalThis.HTMLTableElement {
  caption
  createCaption
  createTBody
  createTFoot
  createTHead
  deleteCaption
  deleteRow
  deleteTFoot
  deleteTHead
  insertRow
  rows
  tBodies
  tFoot
  tHead

  // deprecated
  align
  border
  cellPadding
  cellSpacing
  frame
  rules
  summary
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
