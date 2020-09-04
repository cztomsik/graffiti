import { HTMLElement } from './HTMLElement'

export class HTMLStyleElement extends HTMLElement implements globalThis.HTMLStyleElement {
  get sheet() {
    return Array.from(this.ownerDocument.styleSheets).find(s => s.ownerNode == this) ?? null
  }

  get media() {
    return this.getAttribute('media') ?? ''
  }

  // TODO (this will be in adapter)
  // - connected/disconnected
  // - direct child inserted/removed/data changed
  /*
  _update() {
    const sheet = new CSSStyleSheet(this)
    sheet.insertRules(this.textContent!)

    this._sheet = sheet
  }
  */

  // deprecated
  type
}
