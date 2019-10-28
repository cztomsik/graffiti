import * as assert from 'assert'
import { EventTarget } from '../events/EventTarget'
import { Document } from './Document'
import { Element } from './Element'

export class Node extends EventTarget {
  readonly childNodes: Node[] = []
  parentNode: Node = null

  constructor(public readonly ownerDocument: Document, public readonly nodeType, public readonly _surface) {
    super()
  }

  appendChild(child: Node) {
    return this.insertBefore(child, null)
  }

  insertBefore(child: Node, refNode: Node | null) {
    // should be !== null but some frameworks pass undefined too
    if (refNode) {
      assert.equal(refNode.parentNode, this)
    }

    if (child.nodeType === Node.DOCUMENT_FRAGMENT_NODE) {
      child.childNodes.splice(0).forEach(c => this.appendChild(c))
    } else {
      // find insertion index for both childNodes & for native
      //
      // TODO: not yet sure if it's better to compute index like this or
      // to pass prevSurface to the native and do it there
      // (possibly in each sub-system)
      // but comments are DOM-specific and it probably shouldn't leak there
      let c, index, nativeOffset = 0, len = this.childNodes.length

      for (index = 0; index < len; index++) {
        if ((c = this.childNodes[index]) === refNode) {
          break
        } else if (c._surface === undefined) {
          nativeOffset--
        }
      }

      child.remove()
      child.parentNode = this
      this.childNodes.splice(index, 0, child)

      if (child.nodeType === Node.TEXT_NODE) {
        (this as any)._updateText()
      }

      // comment/text, insert into fragment
      // undefined is needed because root is 0
      if ((child._surface !== undefined) && (this._surface !== undefined)) {
        this.ownerDocument._scene.insertAt(this._surface, child._surface, index + nativeOffset)
      }
    }

    return child
  }

  remove() {
    if (this.parentNode) {
      this.parentNode.removeChild(this)
    }
  }

  removeChild(child: Node) {
    assert.equal(child.parentNode, this)

    this.childNodes.splice(this.childNodes.indexOf(child), 1)

    if (child._surface) {
      this.ownerDocument._scene.removeChild(this._surface, child._surface)
    }

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
      case Node.ELEMENT_NODE: return node.tagName
      case Node.DOCUMENT_FRAGMENT_NODE: return '#document-fragment'
      case Node.DOCUMENT_NODE: return '#document'
      case Node.TEXT_NODE: return '#text'
      case Node.COMMENT_NODE: return '#comment'
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

function sibling(parent, child, offset) {
  return parent && parent.childNodes[parent.childNodes.indexOf(child) + offset]
}
