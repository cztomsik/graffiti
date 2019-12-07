import { Node } from './Node'
import { Text } from './Text'
import { camelCase, EMPTY_OBJ } from '../core/utils'
import { Document } from './Document'
import { CSSStyleDeclaration } from '../styles/CSSStyleDeclaration'

export class Element extends Node {
  id?
  style = new CSSStyleDeclaration(this.ownerDocument._scene, this._surface)
  // preact needs this sometimes
  attributes = []

  constructor(public ownerDocument: Document, public tagName, _surface) {
    super(ownerDocument, Node.ELEMENT_NODE, _surface)
  }

  // so the events can bubble
  // @see EventTarget
  _getTheParent() {
    return this.parentElement
  }

  _updateText() {
    // this is very ugly temporary hack just to have something working
    // we dont support mixing text & elements yet so we
    // just set the text to the concatenated result
    let content = '', len = this.childNodes.length
    for (let i = 0; i < len; i++) {
      const c = this.childNodes[i]

      if (c.nodeType === Node.TEXT_NODE) {
        content += (c as Text)._data
      }
    }

    this.style['content'] = content
  }

  setAttribute(name, value) {
    this[camelCase(name)] = value
  }

  removeAttribute(name) {
    delete this[camelCase(name)]
  }

  blur() {
    if (this.ownerDocument.activeElement !== this) {
      return
    }

    this._fire('blur')
    this.ownerDocument.activeElement = null
  }

  focus() {
    if (this.ownerDocument.activeElement === this) {
      return
    }

    if (this.ownerDocument.activeElement) {
      this.ownerDocument.activeElement.blur()
    }

    this.ownerDocument.activeElement = this
    this._fire('focus')
  }

  querySelector(selectors: string): Element | null {
    return this.querySelectorAll(selectors)[0] || null
  }

  // TODO: sizzle.js?
  querySelectorAll(selectors: string): Element[] {
    return []
  }

  getBoundingClientRect() {
    // TODO: DOMRect
    const [[left, top], [bottom, right]] = this.ownerDocument._scene.getBounds(this._surface)

    // TODO: spec allows negative width/height
    return { x: left, y: top, left, top, bottom, right, width: right - left, height: bottom - top }
  }

  get offsetWidth() {
    return 0
  }

  get offsetHeight() {
    return 0
  }

  set textContent(v) {
    this.childNodes.forEach(c => c.remove())

    this.appendChild(this.ownerDocument.createTextNode(v))

    this._updateText()
  }
}
