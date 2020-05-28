import { Node } from './Node'
import { camelCase, ERR } from '../util'
import { NodeList } from './NodeList'

export abstract class Element extends Node implements globalThis.Element {
  childNodes = new NodeList<ChildNode>()
  _localName: string
  abstract readonly tagName: string;

  // TODO
  attributes: any = []

  constructor(doc = document, localName: string = ERR('new Element() is not supported')) {
    super(doc)

    this._localName = localName

    this.ownerDocument._initElement(this, localName)
  }

  _insertChildAt(child, index) {
    super._insertChildAt(child, index)

    this.ownerDocument._elChildInserted(this, child, index)
  }

  removeChild<T extends Node>(child: T): T {
    super.removeChild(child)

    this.ownerDocument._elChildRemoved(this, child)

    return child
  }

  get localName() {
    return this._localName
  }

  get nodeType() {
    return Node.ELEMENT_NODE
  }

  get nodeName() {
    return this.tagName
  }

  getAttribute(name: string): string | null {

  }

  setAttribute(name: string, value: string) {
    this[camelCase(name)] = value as any
  }

  removeAttribute(name: string) {
    delete this[camelCase(name)]
  }

  // TODO: replace `:scope` with this.tagName
  //       https://www.w3.org/TR/selectors-4/#the-scope-pseudo
  querySelector(selectors: string): Element | null {
    return this.ownerDocument.querySelector(selectors, this)
  }

  querySelectorAll(selectors) {
    return this.ownerDocument.querySelectorAll(selectors, this)
  }

  get id() {
    return this.getAttribute('id')
  }

  set id(id: string) {
    this.setAttribute('id', id)
  }

  get className() {
    return this.getAttribute('class')
  }

  set className(className: string) {
    this.setAttribute('class', className)
  }

  // so the events can bubble
  // @see EventTarget
  _getTheParent() {
    return this.parentElement
  }

  // later
  // (outerHTML should fail on `doc.documentElement`)
  innerHTML
  outerHTML
  scrollLeft
  scrollTop
  getBoundingClientRect

  // maybe later
  animate
  assignedSlot
  attachShadow
  classList
  clientHeight
  clientLeft
  clientTop
  clientWidth
  closest
  getAnimations
  getAttributeNames
  getAttributeNode
  getAttributeNodeNS
  getAttributeNS
  getClientRects
  getElementsByTagName
  getElementsByTagNameNS
  getElementsByClassName
  hasAttribute
  hasAttributeNS
  hasAttributes
  hasPointerCapture
  insertAdjacentElement
  insertAdjacentHTML
  insertAdjacentText
  matches
  msGetRegionContent
  namespaceURI
  prefix
  releasePointerCapture
  removeAttributeNode
  removeAttributeNS
  requestFullscreen
  requestPointerLock
  scroll
  scrollBy
  scrollHeight
  scrollIntoView
  scrollTo
  scrollWidth
  setAttributeNode
  setAttributeNodeNS
  setAttributeNS
  setPointerCapture
  shadowRoot
  slot
  toggleAttribute
  webkitMatchesSelector
}
