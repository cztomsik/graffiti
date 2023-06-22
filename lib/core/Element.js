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

  get nodeName() {
    return this.tagName
  }

  get tagName() {
    return this.localName
  }

  get localName() {
    return native.Element_local_name(this)
  }

  get style() {
    if (!this._style) {
      this._style = wrap(CSSStyleDeclaration, native.Element_style(this))
      Object.defineProperty(this._style, 'ownerNode', { value: this })
    }

    return this._style
  }

  /** @deprecated */
  get attributes() {
    // preact needs this
    // otherwise we really don't want to support Attr & NamedNodeMap because
    // it would only make everything much more complex with no real benefit
    // if we'll ever need it, it should be lazy-created weak-stored proxy
    // and it should still delegate to el.get/setAttribute()
    return this.getAttributeNames().map(name => ({ name, value: this.getAttribute(name) }))
  }

  getAttribute(name) {
    return native.Element_getAttribute(this, name)
  }

  getAttributeNames() {
    return native.Element_getAttributeNames(this)
  }

  hasAttribute(name) {
    return native.Element_hasAttribute(this, name)
  }

  hasAttributes() {
    return native.Element_hasAttributes(this)
  }

  setAttribute(name, value) {
    native.Element_setAttribute(this, name, '' + value)
  }

  removeAttribute(name) {
    native.Element_removeAttribute(this, name)
  }

  toggleAttribute(name, force = false) {
    if (this.hasAttribute(name)) {
      if (force) {
        return true
      }

      this.removeAttribute(name)
      return false
    }

    if (!force) {
      return false
    }

    this.setAttribute(name, '')
    return true
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

  get innerText() {
    // TODO: return native.Element_innerText(this)
    return this.textContent
  }

  set innerText(innerText) {
    this.textContent = innerText
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

  matches(selector) {
    return native.Element_matches(this, selector)
  }

  closest(selector) {
    this.matches(selector) ? this : this.parentElement?.closest(selector)
  }
}
