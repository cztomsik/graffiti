import { HTMLElement } from './index'
import { DOMParser } from '../dom/DOMParser'
import { TODO } from '../util'

// this exists mostly for testing querySelectorAll() in WPT test harness
// (but it doesn't work anyway)
export class HTMLIFrameElement extends HTMLElement implements globalThis.HTMLIFrameElement {
  #contentDocument = new DOMParser().parseFromString('', 'text/html')

  get contentDocument() {
    return this.#contentDocument
  }

  get contentWindow() {
    return TODO()
  }

  get name() {
    return this.getAttribute('name') ?? ''
  }

  set name(name) {
    this.setAttribute('name', name)
  }

  get src() {
    return this.getAttribute('src') ?? ''
  }

  set src(src) {
    this.setAttribute('src', src)
  }

  get width() {
    return this.getAttribute('width') ?? ''
  }

  set width(width) {
    this.setAttribute('width', width)
  }

  get height() {
    return this.getAttribute('height') ?? ''
  }

  set height(height) {
    this.setAttribute('height', height)
  }

  // deprecated
  frameBorder
  align
  longDesc
  scrolling
  marginHeight
  marginWidth

  // later
  allow
  allowFullscreen
  allowPaymentRequest
  referrerPolicy
  sandbox
  srcdoc
  getSVGDocument
  nonce?
  version
}
