import { HTMLElement } from './index'

export class HTMLTableRowElement extends HTMLElement implements globalThis.HTMLTableRowElement {
  cells
  rowIndex
  sectionRowIndex
  insertCell
  deleteCell
  
  // deprecated
  align
  bgColor
  ch
  chOff
  vAlign
}
