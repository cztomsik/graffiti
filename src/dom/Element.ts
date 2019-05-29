import { Node } from './Node'
import { camelCase, EMPTY_OBJ } from '../core/utils'
import { Document } from './Document'
import { diffStyle } from '../styles/diff'

export class Element extends Node {
  id?
  _style = EMPTY_OBJ

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

  _updateStyle(style) {
    for (const styleProp of diffStyle(style, this._style)) {
      this.ownerDocument._scene.setStyleProp(this._nativeId, styleProp)
    }
    this._style = style
  }

  // minimal impl just to get preact working
  get style() {
    return new Proxy(
      { setProperty: (prop, value) => this._updateStyle({ ...this._style, [camelCase(prop)]: value }) },
      {
        set: (target, prop, value) => (target.setProperty(prop, value), true)
      }
    )
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
