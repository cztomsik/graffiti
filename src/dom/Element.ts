import { Node } from './Node'
import { Text } from './Text'
import { camelCase, EMPTY_OBJ } from '../core/utils'
import { Document } from './Document'
import { CSSStyleDeclaration } from '../styles/CSSStyleDeclaration'

export class Element extends Node {
  id?
  style = new CSSStyleDeclaration(this.ownerDocument._scene, this._nativeId)
  // preact needs this sometimes
  attributes = []

  constructor(public ownerDocument: Document, public tagName, _nativeId) {
    super(ownerDocument, Node.ELEMENT_NODE, _nativeId)
  }

  // so the events can bubble
  // @see EventTarget
  _getTheParent() {
    return this.parentElement
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

  // TODO: display: none
  get offsetParent() {
    return this.parentElement
  }

  get offsetLeft() {
    const [[left]] = this._bounds

    return left
  }

  get offsetTop() {
    const [[, top]] = this._bounds

    return top
  }

  get offsetWidth() {
    const [[left], [right]] = this._bounds

    return right - left
  }

  get offsetHeight() {
    const [[, top], [, bottom]] = this._bounds

    return bottom - top
  }

  // TODO
  get scrollLeft() {
    return 0
  }

  // TODO
  get scrollTop() {
    return 0
  }

  // TODO: "relative" to the viewport (excluding scrollX, scrollY)
  getBoundingClientRect() {
    // TODO: DOMRect
    const [[left, top], [bottom, right]] = this._bounds

    // TODO: spec allows negative width/height
    return { x: left, y: top, left, top, bottom, right, width: right - left, height: bottom - top }
  }

  get _bounds() {
    return this.ownerDocument._scene.getOffsetBounds(this._nativeId)
  }

  set textContent(v) {
    if ((this.childNodes.length) === 1 && (this.childNodes[0].nodeType === Node.TEXT_NODE)) {
      (this.childNodes[0] as Text).data = v
      return
    }

    this.childNodes.forEach(c => c.remove())

    this.appendChild(this.ownerDocument.createTextNode(v))
  }
}
