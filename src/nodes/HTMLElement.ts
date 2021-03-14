import { Element } from './index'
import { CSSStyleDeclaration } from '../css/CSSStyleDeclaration'

export abstract class HTMLElement extends Element implements globalThis.HTMLElement {
  readonly style = new CSSStyleDeclaration(null, (prop, value) => console.log('.style change', prop, value))

  get tagName() {
    return this.localName.toUpperCase()
  }

  getAttribute(name: string): string | null {
    if (name === 'style') {
      return this.style.cssText
    }

    return super.getAttribute(name)
  }

  setAttribute(name: string, value: string) {
    if (name === 'style') {
      this.style.cssText = value
      return
    }

    super.setAttribute(name, value)
  }

  removeAttribute(name: string) {
    if (name === 'style') {
      this.style.cssText = ''
      return
    }

    super.removeAttribute(name)
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

    if (this.ownerDocument.activeElement instanceof HTMLElement) {
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
  enterKeyHint
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
