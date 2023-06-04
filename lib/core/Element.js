// TODO: https://github.com/cztomsik/graffiti/blob/bd1dfe61d3d7b5bfbf9184ecfb9e068dda982a60/src/dom/Element.ts

import { native, wrap } from './native.js'
import { Node } from './Node.js'
import { CSSStyleDeclaration } from './CSSStyleDeclaration.js'

export class Element extends Node {
  _style = null

  get nodeType() {
    return Node.ELEMENT_NODE
  }

  get style() {
    return this._style || (this._style = wrap(CSSStyleDeclaration, native.Element_style(this)))
  }

  getAttribute(name) {
    return native.Element_getAttribute(this, name)
  }

  setAttribute(name, value) {
    native.Element_setAttribute(this, name, '' + value)
  }

  removeAttribute(name) {
    native.Element_removeAttribute(this, name)
  }

  get id() {
    return this.getAttribute('id') ?? ''
  }

  set id(id) {
    this.setAttribute('id', id)
  }

  get className() {
    return this.getAttribute('class') ?? ''
  }

  set className(className) {
    this.setAttribute('class', className)
  }

  get innerHTML() {
    return ''
    // const s = new XMLSerializer()
    // return this.childNodes.map(n => s.serializeToString(n)).join('')
  }

  set innerHTML(html) {
    // TODO: this is only for goober
    if (html === ' ') {
      this.appendChild(document.createTextNode(' '))
      return
    }

    throw new Error('not implemented')

    // this.childNodes.forEach(n => this.removeChild(n))

    // const f = parseFragment(this.ownerDocument, html)
    // this.appendChild(f)
  }
}
