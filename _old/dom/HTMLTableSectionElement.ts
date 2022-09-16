import { HTMLElement } from './index'

export class HTMLTableSectionElement extends HTMLElement implements globalThis.HTMLTableSectionElement {
  deleteRow
  insertRow
  rows

  // deprecated
  align
  ch
  chOff
  vAlign
}
