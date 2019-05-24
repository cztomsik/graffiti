import { EventTarget } from "../events/EventTarget";
import { Document } from "./Document";

export class Node extends EventTarget {
  readonly childNodes: Node[] = []

  constructor(public readonly ownerDocument: Document, public readonly nodeType, public readonly _nativeId) {
    super()
  }

  appendChild(child: Node) {
    this.insertBefore(child, null)
  }

  insertBefore(child: Node, before: Node | null) {
    const index = before === null ?this.childNodes.length :this.childNodes.indexOf(child)

    // consider if it's worth to throw like browsers do
    if (~index) {
      this.insertAt(child, index)
    }
  }

  insertAt(child: Node, index) {
    child.remove()

    // TODO: native.insertAt? (storage is dense anyway)
    if (index === this.childNodes.length) {
      this.ownerDocument._scene.appendChild(this._nativeId, child._nativeId)
    } else {
      this.ownerDocument._scene.insertBefore(this._nativeId, child._nativeId, this.childNodes[index]._nativeId)
    }

    this.childNodes.splice(index, 0, child)
  }

  remove() {
    const parent = this.parentNode

    if (parent) {
      parent.removeChild(this)
    }
  }

  removeChild(child: Node) {
    const index = this.childNodes.indexOf(child)

    // throw?
    if (~index) {
      this.childNodes.splice(index, 1)
      this.ownerDocument._scene.removeChild(this._nativeId, child._nativeId)
    }
  }

  replaceChild(child: Node, prev: Node) {
    const index = this.childNodes.indexOf(prev)

    if (~index) {
      this.insertAt(child, index)
      this.removeChild(prev)
    }
  }

  get firstChild() {
    return this.childNodes[0]
  }

  get lastChild() {
    const chs = this.childNodes

    return chs[chs.length - 1]
  }

  get parentNode() {
    return this.parentElement as Node
  }

  get parentElement() {
    return this.ownerDocument._getParent(this._nativeId)
  }

  get nextSibling() {
    const parentChildren = this.parentElement.childNodes

    return parentChildren[parentChildren.indexOf(this) + 1]
  }

  get previousSibling() {
    const parentChildren = this.parentElement.childNodes

    return parentChildren[parentChildren.indexOf(this) - 1]
  }

  // TODO: get/set nodeValue
  get nodeName() {
    const node = this as any

    switch (this.nodeType) {
      case Node.ELEMENT_NODE: return node.tagName
      case Node.DOCUMENT_NODE: return '#document'
      case Node.TEXT_NODE: return '#text'
    }
  }

  static ELEMENT_NODE = 1
  static TEXT_NODE = 3
  static DOCUMENT_NODE = 9
}
