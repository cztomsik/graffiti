import { FocusEvent, MouseEvent } from '../events/index'
import { Element } from './index'

export abstract class HTMLElement extends Element implements globalThis.HTMLElement {
  get tagName() {
    return this.localName.toUpperCase()
  }

  click() {
    this.dispatchEvent(new MouseEvent('click', { bubbles: true, cancelable: true }))
  }

  blur() {
    if (this.ownerDocument.activeElement !== this) {
      return
    }

    this.dispatchEvent(new FocusEvent('blur'))
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
    this.dispatchEvent(new FocusEvent('focus'))
  }

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
  inputMode
  isContentEditable
  lang
  outerText
  spellcheck
  tabIndex
  title
  translate
}
