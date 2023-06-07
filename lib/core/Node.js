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
    return this.removeChild(child)
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

  querySelector(selector) {
    return native.Node_querySelector(this, selector)
  }

  // node types
  static ELEMENT_NODE = 1
  static TEXT_NODE = 3
  static COMMENT_NODE = 8
  static DOCUMENT_NODE = 9
  static DOCUMENT_FRAGMENT_NODE = 11
}

Object.assign(Node.prototype, {
  previousSibling: null,
  nextSibling: null,
  parentNode: null,
  firstChild: null,
  lastChild: null,
})
