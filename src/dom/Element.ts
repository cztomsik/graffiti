import { Node, NodeList, XMLSerializer } from './index'
import { IElement } from '../types'
import { parseFragment } from './DOMParser'
import { native, atom, encode } from '../native'
import { CSSStyleDeclaration } from '../css/CSSStyleDeclaration'
import { DOMTokenList } from './DOMTokenList'
import { DOC_ID, NODE_ID } from './Document'

export abstract class Element extends Node implements IElement {
  readonly childNodes = new NodeList<ChildNode>()
  #localName: string
  #attributes = {}

  // both lazy-created
  #classList?: DOMTokenList
  #style?: CSSStyleDeclaration

  constructor(localName: string, doc = document) {
    super(doc)

    this.#localName = localName

    this[NODE_ID] = native.gft_Document_create_element(doc[DOC_ID], atom(localName))
  }

  get nodeType() {
    return Node.ELEMENT_NODE
  }

  get nodeName() {
    return this.tagName
  }

  get tagName() {
    return this.localName
  }

  get localName() {
    return this.#localName
  }

  get style() {
    return this.#style ?? (this.#style = new CSSStyleDeclaration(this))
  }

  /** @deprecated */
  get attributes(): any {
    // preact needs this
    // otherwise we really don't want to support Attr & NamedNodeMap because
    // it would only make everything much more complex with no real benefit
    // if we'll ever need it, it should be lazy-created weak-stored proxy
    // and it should still delegate to el.get/setAttribute()
    return this.getAttributeNames().map(name => ({ name, value: this.getAttribute(name) }))
  }

  getAttribute(name: string): string | null {
    return this.#attributes[name] ?? null
  }

  getAttributeNames(): string[] {
    return Object.keys(this.#attributes)
  }

  hasAttribute(name: string): boolean {
    return name in this.#attributes
  }

  hasAttributes(): boolean {
    return !!this.getAttributeNames().length
  }

  setAttribute(name: string, value: string) {
    this.#attributes[name] = value = typeof value === 'string' ? value : '' + value

    native.gft_Document_set_attribute(this.ownerDocument[DOC_ID], this[NODE_ID], atom(name), encode(value))
  }

  removeAttribute(name: string) {
    delete this.#attributes[name]

    native.gft_Document_remove_attribute(this.ownerDocument[DOC_ID], this[NODE_ID], atom(name))
  }

  toggleAttribute(name: string, force?: boolean): boolean {
    if (this.hasAttribute(name)) {
      if (force) {
        return true
      }

      this.removeAttribute(name)
      return false
    }

    if (!force && force !== undefined) {
      return false
    }

    this.setAttribute(name, '')
    return true
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

  get innerText() {
    return this.textContent!
  }

  set innerText(innerText) {
    this.textContent = innerText
  }

  get innerHTML() {
    const s = new XMLSerializer()

    return this.childNodes.map(n => s.serializeToString(n)).join('')
  }

  set innerHTML(html) {
    this.childNodes.forEach(n => this.removeChild(n))

    const f = parseFragment(this.ownerDocument, html)
    this.append(f)
  }

  get outerHTML() {
    return new XMLSerializer().serializeToString(this)
  }

  set outerHTML(html) {
    this.replaceWith(parseFragment(this.ownerDocument, html))
  }

  matches(selector: string): boolean {
    return native.gft_Document_element_matches(this.ownerDocument[DOC_ID], this[NODE_ID], selector)
  }

  closest(selector: string) {
    this.matches(selector) ? this : this.parentElement?.closest(selector)
  }

  get classList() {
    return this.#classList ?? (this.#classList = new DOMTokenList(this, 'className'))
  }

  // later
  scrollLeft
  scrollTop
  getBoundingClientRect

  // maybe later
  animate
  attachInternals
  attachShadow
  clientHeight
  clientLeft
  clientTop
  clientWidth
  getAnimations
  getAttributeNode
  getAttributeNodeNS
  getAttributeNS
  getClientRects
  hasAttributeNS
  hasPointerCapture
  insertAdjacentElement
  insertAdjacentHTML
  insertAdjacentText
  msGetRegionContent
  offsetHeight
  offsetLeft
  offsetParent
  offsetTop
  offsetWidth
  part
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

  // not sure if and when
  ariaAtomic
  ariaAutoComplete
  ariaBusy
  ariaChecked
  ariaColCount
  ariaColIndex
  ariaColSpan
  ariaCurrent
  ariaDisabled
  ariaExpanded
  ariaHasPopup
  ariaHidden
  ariaKeyShortcuts
  ariaLabel
  ariaLevel
  ariaLive
  ariaModal
  ariaMultiLine
  ariaMultiSelectable
  ariaOrientation
  ariaPlaceholder
  ariaPosInSet
  ariaPressed
  ariaReadOnly
  ariaRequired
  ariaRoleDescription
  ariaRowCount
  ariaRowIndex
  ariaRowSpan
  ariaSelected
  ariaSetSize
  ariaSort
  ariaValueMax
  ariaValueMin
  ariaValueNow
  ariaValueText

  // ignore vendor
  webkitMatchesSelector
}
