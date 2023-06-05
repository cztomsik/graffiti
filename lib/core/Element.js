// TODO: https://github.com/cztomsik/graffiti/blob/bd1dfe61d3d7b5bfbf9184ecfb9e068dda982a60/src/dom/Element.ts

import { native, wrap } from './native.js'
import { Node } from './Node.js'
import { CSSStyleDeclaration } from './CSSStyleDeclaration.js'
import { XMLSerializer } from './XMLSerializer.js'
import { HTMLParser } from './htmlparser.js'

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
    const s = new XMLSerializer()
    return this.childNodes.map(n => s.serializeToString(n)).join('')
  }

  set innerHTML(html) {
    this.childNodes.forEach(n => this.removeChild(n))

    // TODO: parseFragment()
    let stack = [this]

    HTMLParser(html, {
      start: (tag, atts) => {
        let el = document.createElement(tag)
        atts.forEach(att => el.setAttribute(att.name, att.value))
        stack[0].appendChild(el)
        stack.unshift(el)
      },
      end: _ => stack.shift(),
      chars: cdata => stack[0].appendChild(document.createTextNode(cdata)),
      comment: cdata => stack[0].appendChild(document.createComment(cdata)),
    })
  }
}
