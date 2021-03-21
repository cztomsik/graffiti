import { HTMLElement } from './index';

export class HTMLScriptElement extends HTMLElement implements globalThis.HTMLScriptElement {
  get src() {
    return this.getAttribute('src') ?? ''
  }

  set src(src) {
    this.setAttribute('src', src)
  }

  get text() {
    return this.textContent ?? ''
  }

  set text(text) {
    this.textContent = text
  }

  get async() {
    return this.hasAttribute('async')
  }

  set async(async) {
    this.toggleAttribute('async', async)
  }

  get defer() {
    return this.hasAttribute('defer')
  }

  set defer(defer) {
    this.toggleAttribute('defer', defer)
  }

  // later
  crossOrigin
  integrity
  noModule
  referrerPolicy
  type
  nonce?

  // deprecated
  charset
  event
  htmlFor
}
