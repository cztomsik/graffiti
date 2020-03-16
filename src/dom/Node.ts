import * as assert from 'assert'
import { EventTarget } from '../events/EventTarget'
import { Document } from './Document'
import { Element } from './Element'

// perf(const)
const ELEMENT_NODE = 1
const TEXT_NODE = 3
const COMMENT_NODE = 8
const DOCUMENT_NODE = 9
const DOCUMENT_FRAGMENT_NODE = 11

export class Node extends EventTarget {
  readonly childNodes: Node[] = []
  parentNode: Node = null

  constructor(public readonly ownerDocument: Document, public readonly nodeType, public _nativeId) {
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

    // fragment
    if (child.nodeType === DOCUMENT_FRAGMENT_NODE) {
      child.childNodes.splice(0).forEach(c => this.insertBefore(c, refNode))
      return child
    }

    // remove first (in case it was in the same element already)
    child.remove()

    // final position
    let index = refNode ? this.childNodes.indexOf(refNode) : this.childNodes.length

    child.parentNode = this
    this.childNodes.splice(index, 0, child)

    // fragment does not have an id
    // overriding it in Element would be nicer but it'd been 1-2 extra calls
    if (this._nativeId !== undefined) {
      const textBefore = index !== 0 && this.childNodes[index - 1].nodeType === TEXT_NODE
      const textAfter = refNode && refNode.nodeType === TEXT_NODE

      switch (child.nodeType) {
        case ELEMENT_NODE:
          this.ownerDocument._scene.insertElementAt(this._nativeId, child._nativeId, index)

          if (textBefore && textAfter) {
            splitTexts(this.childNodes[index - 1], refNode)
          }

          break

        case TEXT_NODE:
          this.ownerDocument._scene.insertTextAt(this._nativeId, child._nativeId, index)

          if (textBefore) {
            joinTexts(this.childNodes[index - 1], child)
          } else if (textAfter) {
            joinTexts(child, refNode)
          } else {
            updateText(child)
          }

          break

        case COMMENT_NODE:
          this.ownerDocument._scene.insertTextAt(this._nativeId, child._nativeId, index)
          break

        default:
          throw new Error('unsupported node type')
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

    const index = this.childNodes.indexOf(child)

    // so that events dont sink in unattached subtree
    if (child.nodeType === ELEMENT_NODE) {
      ;(child as Element).blur()
    }

    // fragment does not have an id
    if (this._nativeId !== undefined) {
      switch (child.nodeType) {
        case ELEMENT_NODE:
          this.ownerDocument._scene.removeElement(this._nativeId, child._nativeId)

          const prev = this.childNodes[index - 1]
          const next = this.childNodes[index + 1]

          if (prev?.nodeType === TEXT_NODE && next?.nodeType === TEXT_NODE) {
            joinTexts(prev, next)
          }

          break

        case TEXT_NODE:
          removeText(child)
          this.ownerDocument._scene.removeText(this._nativeId, child._nativeId)
          break

        case COMMENT_NODE:
          this.ownerDocument._scene.removeText(this._nativeId, child._nativeId)
          break
      }
    }

    this.childNodes.splice(index, 1)
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
    switch (this.nodeType) {
      case ELEMENT_NODE:
        return (this as any).tagName.toUpperCase()
      case DOCUMENT_FRAGMENT_NODE:
        return '#document-fragment'
      case DOCUMENT_NODE:
        return '#document'
      case TEXT_NODE:
        return '#text'
      case COMMENT_NODE:
        return '#comment'
    }
  }

  // TODO: get/set
  // https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeValue
  // (Text.nodeValue exists already)
  get nodeValue() {
    return null
  }

  static ELEMENT_NODE = ELEMENT_NODE
  static TEXT_NODE = TEXT_NODE
  static COMMENT_NODE = COMMENT_NODE
  static DOCUMENT_NODE = DOCUMENT_NODE
  static DOCUMENT_FRAGMENT_NODE = DOCUMENT_FRAGMENT_NODE
}

const sibling = (parent, child, offset) =>
  parent && (parent.childNodes[parent.childNodes.indexOf(child) + offset] || null)

// wouldn't work with import (circular dependency)
// TODO: it would with esModuleInterop
const { joinTexts, updateText, splitTexts, removeText } = require('./Text')
