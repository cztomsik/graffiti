import { Node } from './Node'
import { Text } from './Text'
import { camelCase, EMPTY_OBJ } from '../core/utils'
import { Document } from './Document'
import { CSSStyleDeclaration } from '../styles/CSSStyleDeclaration'

export class Element extends Node {
  id?
  style = new CSSStyleDeclaration(this.ownerDocument._scene, this._nativeId)

  constructor(public ownerDocument: Document, public tagName, public _nativeId) {
    super(ownerDocument, Node.ELEMENT_NODE, _nativeId)
  }

  insertAt(child: Node, index) {
    if (child.nodeType === Node.TEXT_NODE) {
      this._setText((child as Text).data)
    } else if (child.nodeType === Node.DOCUMENT_FRAGMENT_NODE) {
      child.childNodes.forEach((c, i) => this.insertAt(c, index + i))
    } else {
      super.insertAt(child, index)
      this.ownerDocument._scene.insertAt(this._nativeId, child._nativeId, index)
    }
  }

  removeChild(child: Node) {
    super.removeChild(child)
    this.ownerDocument._scene.removeChild(this._nativeId, child._nativeId)
  }

  // so the events can bubble
  // @see EventTarget
  _getTheParent() {
    return this.parentElement
  }

  _setText(text) {
    this.style['content'] = text
  }

  setAttribute(name, value) {
    this[camelCase(name)] = value
  }

  removeAttribute(name) {
    delete this[camelCase(name)]
  }

  querySelector(selectors: string): Element | null {
    return this.querySelectorAll(selectors)[0] || null
  }

  // TODO: sizzle.js?
  querySelectorAll(selectors: string): Element[] {
    return []
  }

  getBoundingClientRect() {
    return { x: 0, left: 0, y: 0, top: 0, width: 100, height: 100 }
  }

  get offsetWidth() {
    return 0
  }

  get offsetHeight() {
    return 0
  }

  set textContent(v) {
    // TODO: check that nobody relies on .childNodes > 0
    this._setText(v)
  }
}
