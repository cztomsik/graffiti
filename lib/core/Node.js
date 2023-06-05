// TODO: https://github.com/cztomsik/graffiti/blob/bd1dfe61d3d7b5bfbf9184ecfb9e068dda982a60/src/dom/Node.ts

import { native } from './native.js'
import { EventTarget } from './EventTarget.js'

export class Node extends EventTarget {
  get parentNode() {
    return native.Node_parent_node(this)
  }

  get firstChild() {
    return native.Node_first_child(this)
  }

  get previousSibling() {
    return native.Node_previous_sibling(this)
  }

  get nextSibling() {
    return native.Node_next_sibling(this)
  }

  // TODO: NodeList
  get childNodes() {
    const res = []
    for (let n = this.firstChild; n; n = n.nextSibling) res.push(n)
    return res
  }

  appendChild(child) {
    return native.Node_appendChild(this, child), child
  }

  insertBefore(child, before) {
    return before ? (native.Node_insertBefore(this, child, before), child) : this.appendChild(child)
  }

  replaceChild(child, oldChild) {
    return this.insertBefore(child, oldChild), this.removeChild(child)
  }

  removeChild(child) {
    return native.Node_removeChild(this, child), child
  }

  // node types
  static ELEMENT_NODE = 1
  static TEXT_NODE = 3
  static COMMENT_NODE = 8
  static DOCUMENT_NODE = 9
  static DOCUMENT_FRAGMENT_NODE = 11
}
