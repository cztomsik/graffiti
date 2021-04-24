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

  // deprecated too but nice to have
  get bgColor() {
    return this.style.getPropertyValue('background-color')
  }

  set bgColor(v) {
    this.style.setPropertyValue('background-color', '' + v)
  }
}
