import { Node } from './Node'
import { Document } from './Document'
import { NodeList } from './NodeList'
import { ERR } from '../util'

export abstract class Element extends Node implements globalThis.Element {
  abstract readonly tagName: string;
  readonly childNodes = new NodeList<ChildNode>()
  _localName: string
  _attributes = new Map<string, string>()

  constructor(doc = document as Document, localName: string = ERR('new Element() is not supported')) {
    super(doc)

    this._localName = localName

    this.ownerDocument._initElement(this, localName)
  }

  get nodeType() {
    return Node.ELEMENT_NODE
  }

  get nodeName() {
    return this.tagName
  }

  get localName() {
    return this._localName
  }

  /** @deprecated */
  get attributes(): any {
    // preact needs this
    // otherwise we really don't want to support Attr & NamedNodeMap because
    // it would only make everything much more complex with no real benefit
    // if we'll ever need it, it should be lazy-created weak-stored proxy
    // and it should still delegate to el.get/setAttribute()
    return Array.from(this._attributes).map(([name, value]) => ({ name, value }))
  }

  getAttribute(name: string): string | null {
    return this._attributes.get(name) ?? null
  }

  getAttributeNames(): string[] {
    return [...this._attributes.keys()]
  }

  hasAttribute(name: string): boolean {
    return this._attributes.has(name)
  }

  hasAttributes(): boolean {
    return !!this.getAttributeNames().length
  }

  setAttribute(name: string, value: string) {
    this._attributes.set(name, value)
  }

  removeAttribute(name: string) {
    this._attributes.delete(name)
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

  // TODO: replace `:scope` with this.tagName
  //       https://www.w3.org/TR/selectors-4/#the-scope-pseudo
  querySelector(selectors: string): Element | null {
    return this.ownerDocument.querySelector(selectors, this)
  }

  querySelectorAll(selectors) {
    return this.ownerDocument.querySelectorAll(selectors, this)
  }

  get id() {
    return this.getAttribute('id') ?? ''
  }

  set id(id: string) {
    this.setAttribute('id', id)
  }

  get className() {
    return this.getAttribute('class') ?? ''
  }

  set className(className: string) {
    this.setAttribute('class', className)
  }

  // so the events can bubble
  // @see EventTarget
  _getTheParent() {
    return this.parentElement as any
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
  getAttributeNode
  getAttributeNodeNS
  getAttributeNS
  getClientRects
  getElementsByTagName
  getElementsByTagNameNS
  getElementsByClassName
  hasAttributeNS
  hasPointerCapture
  insertAdjacentElement
  insertAdjacentHTML
  insertAdjacentText
  matches
  msGetRegionContent
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
