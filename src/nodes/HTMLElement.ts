import { Element } from './Element'
import { CSSStyleDeclaration } from '../styles/CSSStyleDeclaration'

export abstract class HTMLElement extends Element implements globalThis.HTMLElement {
  style = new CSSStyleDeclaration(this)

  get tagName() {
    return this.localName.toUpperCase()
  }

  click() {
    this._fire('click')
  }

  blur() {
    if (this.ownerDocument.activeElement !== this) {
      return
    }

    this._fire('blur')
    // TODO: should be in Document
    ;(this.ownerDocument as any).activeElement = null
  }

  focus() {
    if (this.ownerDocument.activeElement === this) {
      return
    }

    if (this.ownerDocument.activeElement) {
      this.ownerDocument.activeElement.blur()
    }

    // TODO: should be in Document
    ;(this.ownerDocument as any).activeElement = this
    this._fire('focus')
  }

  // TODO
  offsetParent
  offsetLeft
  offsetTop
  offsetWidth
  offsetHeight

  // later
  accessKey
  accessKeyLabel
  autocapitalize
  autofocus
  contentEditable
  dataset
  dir
  draggable
  hidden
  innerText
  inputMode
  isContentEditable
  lang
  spellcheck
  tabIndex
  title
  translate
}
