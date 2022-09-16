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
      this.style.setProperty('width', v.match(/^\d+$/) ?v + 'px' :v)
    }

    // TODO: removeAttribute
  }
}
