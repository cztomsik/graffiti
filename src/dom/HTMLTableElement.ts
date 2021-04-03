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
  bgColor
  border
  cellPadding
  cellSpacing
  frame
  rules
  summary
  width
}
