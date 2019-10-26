import { Node } from './Node'
import { Text } from './Text'
import { camelCase, EMPTY_OBJ } from '../core/utils'
import { Document } from './Document'
import { CSSStyleDeclaration } from '../styles/CSSStyleDeclaration'

export class Element extends Node {
  id?
  style = new CSSStyleDeclaration(this.ownerDocument._scene, this._surface)
  textNodes: Text[] = []

  constructor(public ownerDocument: Document, public tagName, _surface) {
    super(ownerDocument, Node.ELEMENT_NODE, _surface)
  }

  insertBefore(child, before) {
    // this is very ugly temporary hack just to have something working
    // we dont support mixing text & elements yet so we put text nodes
    // separately and just set the text to concatenated result
    if (child.nodeType === Node.TEXT_NODE) {
      // even the order can be wrong
      this.textNodes.push(child)
      child.parentNode = this
      this._updateText()
      return child
    }

    return super.insertBefore(child, before)
  }

  removeChild(child: Node) {
    // similar hack for removals
    if (child.nodeType === Node.TEXT_NODE) {
      this.textNodes = this.textNodes.filter(t => t !== child)
      this._updateText()
      return child
    }

    return super.removeChild(child)
  }

  // so the events can bubble
  // @see EventTarget
  _getTheParent() {
    return this.parentElement
  }

  _updateText() {
    this.style['content'] = this.textNodes.length ?this.textNodes.map(t => t.data).join('') :undefined
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
    this.textNodes = [this.ownerDocument.createTextNode(v)]
    this._updateText()
  }
}
