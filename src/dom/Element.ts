import { Node, NodeList, XMLSerializer } from './index'
import { ERR } from '../util'
import { parseFragment } from './DOMParser'
import { native, getNativeId, register, TRUE } from '../native'
import { CSSStyleDeclaration } from '../css/CSSStyleDeclaration'
import { DOMTokenList } from './DOMTokenList'

export abstract class Element extends Node implements globalThis.Element {
  abstract readonly tagName: string
  readonly childNodes = new NodeList<ChildNode>()
  #localName: string

  // both lazy-created
  #classList?: DOMTokenList
  #style?: CSSStyleDeclaration

  constructor(doc = document, localName: string = ERR('new Element() is not supported')) {
    super(doc)

    this.#localName = localName

    register(this, native.Document_create_element(getNativeId(doc), localName))
  }

  get nodeType() {
    return Node.ELEMENT_NODE
  }

  get nodeName() {
    return this.tagName
  }

  get localName() {
    return this.#localName
  }

  get style() {
    return (
      this.#style ?? (this.#style = register(new CSSStyleDeclaration(null), native.Element_style(getNativeId(this))))
    )
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
    return native.Element_attribute(getNativeId(this), name)
  }

  getAttributeNames(): string[] {
    return native.Element_attribute_names(getNativeId(this))
  }

  hasAttribute(name: string): boolean {
    return this.getAttribute(name) !== null
  }

  hasAttributes(): boolean {
    return !!this.getAttributeNames().length
  }

  setAttribute(name: string, value: string) {
    value = (typeof value === 'string' ? value : '' + value).toLowerCase()

    native.Element_set_attribute(getNativeId(this), name, value)
  }

  removeAttribute(name: string) {
    native.Element_remove_attribute(getNativeId(this), name)
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
    return native.Element_matches(getNativeId(this), selector) === TRUE
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
  attachShadow
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
  hasAttributeNS
  hasPointerCapture
  insertAdjacentElement
  insertAdjacentHTML
  insertAdjacentText
  msGetRegionContent
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
