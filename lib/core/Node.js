// TODO: https://github.com/cztomsik/graffiti/blob/bd1dfe61d3d7b5bfbf9184ecfb9e068dda982a60/src/dom/Node.ts

import { native } from './native.js'
import { EventTarget } from './EventTarget.js'

export class Node extends EventTarget {
  // TODO: NodeList
  get childNodes() {
    const res = []
    for (let n = this.firstChild; n; n = n.nextSibling) res.push(n)
    return res
  }

  appendChild(child) {
    native.Node_appendChild(this, child)

    if (this.lastChild) {
      this.lastChild.nextSibling = child
      child.previousSibling = this.lastChild
    } else {
      this.firstChild = child
    }

    this.lastChild = child
    child.parentNode = this

    return child
  }

  insertBefore(child, before) {
    if (!before) {
      return this.appendChild(child)
    }

    if (before.previousSibling) {
      before.previousSibling.nextSibling = child
      child.previousSibling = before.previousSibling
    } else {
      this.firstChild = child
    }

    child.nextSibling = before
    before.previousSibling = child
    child.parentNode = this

    native.Node_insertBefore(this, child, before)
    return child
  }

  replaceChild(child, oldChild) {
    this.insertBefore(child, oldChild)
    return this.removeChild(oldChild)
  }

  removeChild(child) {
    native.Node_removeChild(this, child)

    if (child.previousSibling) {
      child.previousSibling.nextSibling = child.nextSibling
    } else {
      this.firstChild = child.nextSibling
    }

    if (child.nextSibling) {
      child.nextSibling.previousSibling = child.previousSibling
    } else {
      this.lastChild = child.previousSibling
    }

    child.nextSibling = null
    child.previousSibling = null
    child.parentNode = null

    return child
  }

  hasChildNodes() {
    return this.childNodes.length > 0
  }

  get parentElement() {
    return this.parentNode instanceof HTMLElement ? this.parentNode : null
  }

  // overridden by CharacterData
  // comment.textContent should return a value but it
  // shouldn't be part of element.textContent
  get textContent() {
    return this.childNodes
      .filter(c => c.nodeType == Node.ELEMENT_NODE || c.nodeType == Node.TEXT_NODE)
      .map(c => c.textContent)
      .join('')
  }

  // overridden by CharacterData
  set textContent(v) {
    this.childNodes.forEach(c => c.remove())

    // note we can't just update already present text node because it has to remain untouched
    this.appendChild(this.ownerDocument.createTextNode('' + v))
  }

  get isConnected() {
    return this.parentNode?.isConnected ?? false
  }

  contains(other) {
    while (other) {
      if (other === this) {
        return true
      }

      other = other.parentNode
    }

    return false
  }

  // ---
  // ParentNode:

  get children() {
    // TODO: HTMLCollection
    return this.childNodes.filter(c => c.nodeType === Node.ELEMENT_NODE)
  }

  get childElementCount() {
    return this.children.length
  }

  get firstElementChild() {
    return this.children[0] ?? null
  }

  get lastElementChild() {
    const { children } = this
    return children[children.length - 1] ?? null
  }

  getElementById(id) {
    return this.querySelector(`#${id}`)
  }

  querySelector(selector) {
    return native.Node_querySelector(this, selector)
  }

  querySelectorAll(selector) {
    return native.Node_querySelectorAll(this, selector)
  }

  getElementsByTagName(tagName) {
    return this.querySelectorAll(tagName)
  }

  getElementsByClassName(className) {
    return this.querySelectorAll(`.${className}`)
  }

  // ---
  // NonDocumentTypeChildNode:

  get nextElementSibling() {
    return (
      this.nextSibling &&
      (this.nextSibling.nodeType === Node.ELEMENT_NODE ? this.nextSibling : this.nextSibling.nextElementSibling)
    )
  }

  get previousElementSibling() {
    return (
      this.previousSibling &&
      (this.previousSibling.nodeType === Node.ELEMENT_NODE
        ? this.previousSibling
        : this.previousSibling.previousElementSibling)
    )
  }

  // node types
  static ELEMENT_NODE = 1
  static TEXT_NODE = 3
  static COMMENT_NODE = 8
  static DOCUMENT_NODE = 9
  static DOCUMENT_FRAGMENT_NODE = 11
}

Node.prototype.parentNode = null
Node.prototype.firstChild = null
Node.prototype.lastChild = null
Node.prototype.previousSibling = null
Node.prototype.nextSibling = null
