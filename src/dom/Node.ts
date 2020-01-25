import * as assert from 'assert'
import { EventTarget } from '../events/EventTarget'
import { Document } from './Document'
import { Element } from './Element'

export class Node extends EventTarget {
  readonly childNodes: Node[] = []
  parentNode: Node = null

  constructor(public readonly ownerDocument: Document, public readonly nodeType, public readonly _nativeId) {
    super()
  }

  appendChild(child: Node) {
    return this.insertBefore(child, null)
  }

  insertBefore(child: Node, refNode: Node | null) {
    // should be !== null but some libs pass undefined too
    if (refNode) {
      assert.equal(refNode.parentNode, this)
    }

    if (child.nodeType === Node.DOCUMENT_FRAGMENT_NODE) {
      child.childNodes.splice(0).forEach(c => this.appendChild(c))
      return child
    }

    let index = refNode ? this.childNodes.indexOf(refNode) : this.childNodes.length

    child.remove()

    switch (child.nodeType) {
      case Node.ELEMENT_NODE:
        this.ownerDocument._scene.insertElementAt(this._nativeId, child._nativeId, index)
        break

      case Node.TEXT_NODE:
      case Node.COMMENT_NODE:
        this.ownerDocument._scene.insertTextAt(this._nativeId, child._nativeId, index)
        break

      default:
        throw new Error('unsupported node type')
    }

    child.parentNode = this
    this.childNodes.splice(index, 0, child)

    return child
  }

  remove() {
    if (this.parentNode) {
      this.parentNode.removeChild(this)
    }
  }

  removeChild(child: Node) {
    assert.equal(child.parentNode, this)

    // so that events dont sink in unattached subtree
    if (child.nodeType === Node.ELEMENT_NODE) {
      ;(child as Element).blur()
    }

    switch (child.nodeType) {
      case Node.ELEMENT_NODE:
        this.ownerDocument._scene.removeElement(this._nativeId, child._nativeId)
        break

      case Node.TEXT_NODE:
      case Node.COMMENT_NODE:
        this.ownerDocument._scene.removeText(this._nativeId, child._nativeId)
    }

    this.childNodes.splice(this.childNodes.indexOf(child), 1)
    return child
  }

  replaceChild(child: Node, prev: Node) {
    this.insertBefore(child, prev)
    this.removeChild(prev)
  }

  get firstChild() {
    return this.childNodes[0] || null
  }

  get lastChild() {
    return this.childNodes[this.childNodes.length - 1] || null
  }

  get parentElement() {
    return this.parentNode as Element
  }

  get nextSibling() {
    return sibling(this.parentNode, this, 1)
  }

  get previousSibling() {
    return sibling(this.parentNode, this, -1)
  }

  get nodeName() {
    const node = this as any

    switch (this.nodeType) {
      case Node.ELEMENT_NODE:
        return node.tagName.toUpperCase()
      case Node.DOCUMENT_FRAGMENT_NODE:
        return '#document-fragment'
      case Node.DOCUMENT_NODE:
        return '#document'
      case Node.TEXT_NODE:
        return '#text'
      case Node.COMMENT_NODE:
        return '#comment'
    }
  }

  // TODO: get/set
  // https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeValue
  // (Text.nodeValue exists already)
  get nodeValue() {
    return null
  }

  static ELEMENT_NODE = 1
  static TEXT_NODE = 3
  static COMMENT_NODE = 8
  static DOCUMENT_NODE = 9
  static DOCUMENT_FRAGMENT_NODE = 11
}

const sibling = (parent, child, offset) => parent && parent.childNodes[parent.childNodes.indexOf(child) + offset]
