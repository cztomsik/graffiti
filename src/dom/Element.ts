import { Node } from './Node'
import { camelCase, EMPTY_OBJ } from '../core/utils'
import { Document } from './Document'
import { CSSStyleDeclaration } from '../styles/CSSStyleDeclaration'

export class Element extends Node {
  id?
  style = new CSSStyleDeclaration(this.ownerDocument._scene, this._nativeId)

  constructor(public ownerDocument: Document, public tagName, public _nativeId) {
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
}
