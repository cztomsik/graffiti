import { Node } from './Node'
import { Text } from './Text'
import { camelCase } from '../core/utils'

export class Element extends Node implements globalThis.Element {
  id: string

  // preact needs this sometimes
  // TODO: getter with proxy?
  attributes: any = []

  constructor(doc, public readonly tagName: string) {
    super(doc, Node.ELEMENT_NODE)

    this._nativeId = doc._scene.createElement()
    doc._els.push(this)
  }

  get localName() {
    return this.tagName.toLowerCase()
  }

  // so the events can bubble
  // @see EventTarget
  _getTheParent() {
    return this.parentElement
  }

  setAttribute(name: string, value: string) {
    this[camelCase(name)] = value as any
  }

  removeAttribute(name: string) {
    delete this[camelCase(name)]
  }

  blur() {
    if (this.ownerDocument.activeElement !== this) {
      return
    }

    this._fire('blur')
    this.ownerDocument.activeElement = null
  }

  focus() {
    if (this.ownerDocument.activeElement === this) {
      return
    }

    if (this.ownerDocument.activeElement) {
      this.ownerDocument.activeElement.blur()
    }

    this.ownerDocument.activeElement = this
    this._fire('focus')
  }

  querySelector(selectors: string): Element | null {
    return this.querySelectorAll(selectors)[0] || null
  }

  // TODO: sizzle.js?
  querySelectorAll(selectors) {
    return [] as any
  }

  // TODO
  get scrollLeft(): number {
    return 0
  }

  // TODO
  get scrollTop(): number {
    return 0
  }

  // TODO: "relative" to the viewport (excluding scrollX, scrollY)
  getBoundingClientRect() {
    // TODO: DOMRect
    const [[left, top], [bottom, right]] = this._bounds

    // TODO: spec allows negative width/height
    return { x: left, y: top, left, top, bottom, right, width: right - left, height: bottom - top } as any
  }

  get _bounds() {
    return this.ownerDocument._scene.getOffsetBounds(this._nativeId)
  }

  set textContent(v) {
    if ((this.childNodes.length) === 1 && (this.childNodes[0].nodeType === Node.TEXT_NODE)) {
      (this.childNodes[0] as Text).data = v
      return
    }

    this.childNodes.forEach(c => c.remove())

    this.appendChild(this.ownerDocument.createTextNode(v))
  }

  // maybe later
  animate
  assignedSlot
  attachShadow
  classList
  className
  clientHeight
  clientLeft
  clientTop
  clientWidth
  closest
  getAnimations
  getAttribute
  getAttributeNames
  getAttributeNode
  getAttributeNodeNS
  getAttributeNS
  getClientRects
  hasAttribute
  hasAttributeNS
  hasAttributes
  hasPointerCapture
  innerHTML
  insertAdjacentElement
  insertAdjacentHTML
  insertAdjacentText
  matches
  msGetRegionContent
  namespaceURI
  outerHTML
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
