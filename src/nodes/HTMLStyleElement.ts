import { HTMLElement } from './HTMLElement'
import { CSSStyleSheet } from '../css/CSSStyleSheet'

export class HTMLStyleElement extends HTMLElement implements globalThis.HTMLStyleElement {
  sheet = new CSSStyleSheet(this)

  get media() {
    return this.getAttribute('media') ?? ''
  }

  // deprecated
  type
}
