import { HTMLElement } from './HTMLElement'

export class HTMLStyleElement extends HTMLElement implements globalThis.HTMLStyleElement {
  sheet: CSSStyleSheet | null = null

  get media() {
    return this.getAttribute('media') ?? ''
  }

  // later
  nonce?: string | undefined

  // deprecated
  type
}
