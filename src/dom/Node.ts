import * as assert from 'assert'
import { EventTarget } from '../events/EventTarget'
import { Document } from './Document'
import { NodeList } from './NodeList'
import { last } from '../core/utils'

type INode = globalThis.Node

// perf(const vs. property lookup)
const ELEMENT_NODE = 1
const TEXT_NODE = 3
const COMMENT_NODE = 8
const DOCUMENT_NODE = 9
const DOCUMENT_FRAGMENT_NODE = 11

// note it intentionally implements/declares some extra props/meths
// to reduce code-duplication
export abstract class Node extends EventTarget implements INode {
  _nativeId

  readonly childNodes = new NodeList<ChildNode>()
  parentNode: INode & ParentNode | null = null

  constructor(public readonly ownerDocument: Document, public readonly nodeType: number) {
    super()
  }

  appendChild<T extends INode>(child: T): T {
    return this.insertBefore(child, null)
  }

  insertBefore<T extends INode>(child: T, refNode: INode | null): T {
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
    ;(child as any).remove()

    // final position
    let index = refNode ? this.childNodes.indexOf(refNode) : this.childNodes.length

    ;(child as any).parentNode = this
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

  removeChild<T extends INode>(child: T): T {
    assert.equal(child.parentNode, this)

    const index = this.childNodes.indexOf(child)

    // so that events dont sink in unattached subtree
    if (child.nodeType === ELEMENT_NODE) {
      ;(child as any).blur()
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

  //replaceChild<T extends INode>(child: T, oldChild: T): T {
  replaceChild(child, oldChild) {
      this.insertBefore(child, oldChild)

    return this.removeChild(oldChild)
  }

  get firstChild(): ChildNode | null {
    return this.childNodes[0] || null
  }

  get lastChild(): ChildNode | null {
    return last(this.childNodes)
  }

  get parentElement(): HTMLElement | null {
    return this.parentNode as any
  }

  get nextSibling(): ChildNode | null {
    return sibling(this.parentNode, this, 1)
  }

  get previousSibling(): ChildNode | null {
    return sibling(this.parentNode, this, -1)
  }

  get nodeName(): string {
    switch (this.nodeType) {
      case ELEMENT_NODE:
        return this['tagName']
      case TEXT_NODE:
        return '#text'
      case COMMENT_NODE:
        return '#comment'
      case DOCUMENT_NODE:
        return '#document'
      case DOCUMENT_FRAGMENT_NODE:
        return '#document-fragment'
    }
  }

  // TODO: get/set
  // https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeValue
  // (Text.nodeValue exists already)
  get nodeValue(): string | null {
    return null
  }

  get children(): HTMLCollection {
    // TODO: HTMLCollection
    return this.childNodes.filter(c => c.nodeType === ELEMENT_NODE) as any
  }

  get childElementCount(): number {
    return this.children.length
  }

  get firstElementChild(): Element | null {
    return this.children[0]
  }

  get lastElementChild(): Element | null {
    return last(this.children) || null
  }

  isSameNode(node): boolean {
    return node === this
  }

  hasChildNodes(): boolean {
    return this.childNodes.length !== 0
  }

  append(...nodes: (Node | string)[]) {
    nodes.forEach(n => this.appendChild(this._strToNode(n)))
  }

  prepend(...nodes: (Node | string)[]) {
    nodes.forEach(n => this.insertBefore(this._strToNode(n), this.firstChild))
  }

  after(...nodes: (Node | string)[]) {
    const refNode = this.nextSibling

    nodes.forEach(n => this.parentNode.insertBefore(this._strToNode(n), refNode))
  }

  before(...nodes: (Node | string)[]) {
    nodes.forEach(n => this.parentNode.insertBefore(this._strToNode(n), this))
  }

  replaceWith(...nodes: (Node | string)[]) {
    this.before(...nodes)
    this.remove()
  }

  _strToNode(n) {
    return (typeof n === 'string') ? this.ownerDocument.createTextNode('' + n) : n
  }

  static ELEMENT_NODE = ELEMENT_NODE
  static TEXT_NODE = TEXT_NODE
  static COMMENT_NODE = COMMENT_NODE
  static DOCUMENT_NODE = DOCUMENT_NODE
  static DOCUMENT_FRAGMENT_NODE = DOCUMENT_FRAGMENT_NODE

  // later
  contains
  getElementsByClassName
  getElementsByTagName
  isEqualNode
  nextElementSibling
  previousElementSibling
  textContent

  // maybe later
  appendData
  assignedSlot
  baseURI
  cloneNode
  compareDocumentPosition
  deleteData
  getElementsByTagNameNS
  getRootNode
  insertData
  isConnected
  isDefaultNamespace
  length
  lookupNamespaceURI
  lookupPrefix
  namespaceURI
  normalize
  replaceData
  splitText
  substringData
  wholeText

  // (later) it's both static & instance
  ELEMENT_NODE
  ATTRIBUTE_NODE
  TEXT_NODE
  CDATA_SECTION_NODE
  ENTITY_REFERENCE_NODE
  ENTITY_NODE
  PROCESSING_INSTRUCTION_NODE
  COMMENT_NODE
  DOCUMENT_NODE
  DOCUMENT_TYPE_NODE
  DOCUMENT_FRAGMENT_NODE
  NOTATION_NODE

  // ?
  DOCUMENT_POSITION_CONTAINED_BY
  DOCUMENT_POSITION_CONTAINS
  DOCUMENT_POSITION_DISCONNECTED
  DOCUMENT_POSITION_FOLLOWING
  DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC
  DOCUMENT_POSITION_PRECEDING
}

// so common it's easier to just tell TS about it
declare global {
  interface Node {
    _nativeId: number
  }
}

const sibling = (parent, child, offset) =>
  parent && (parent.childNodes[parent.childNodes.indexOf(child) + offset] || null)

// wouldn't work with import (circular dependency)
// TODO: it would with esModuleInterop
const { joinTexts, updateText, splitTexts, removeText } = require('./Text')
