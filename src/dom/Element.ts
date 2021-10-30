import { Node, NodeList, XMLSerializer } from './index'
import { ERR } from '../util'
import { parseFragment } from './DOMParser'
import { native, encode, getNativeId, register, decode, getRefs } from '../native'
import { CSSStyleDeclaration } from '../css/CSSStyleDeclaration'
import { DOMTokenList } from './DOMTokenList'
import { registerElement } from './Document'

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

    const ref = native.gft_Document_create_element(getNativeId(doc), ...encode(localName))
    register(this, ref)
    registerElement(doc, native.gft_Node_id(ref), this)
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
      this.#style ?? (this.#style = register(new CSSStyleDeclaration(null), native.gft_Element_style(getNativeId(this))))
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
    return decode(native.gft_Element_attribute(getNativeId(this), ...encode(name)))
  }

  getAttributeNames(): string[] {
    const refs = getRefs(native.gft_Element_attribute_names(getNativeId(this)))

    // decode() does the drop here
    const names = refs.map(decode)

    return names as any
  }

  hasAttribute(name: string): boolean {
    return this.getAttribute(name) !== null
  }

  hasAttributes(): boolean {
    return !!this.getAttributeNames().length
  }

  setAttribute(name: string, value: string) {
    value = (typeof value === 'string' ? value : '' + value).toLowerCase()

    native.gft_Element_set_attribute(getNativeId(this), ...encode(name), ...encode(value))
  }

  removeAttribute(name: string) {
    native.gft_Element_remove_attribute(getNativeId(this), ...encode(name))
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
    return native.gft_Element_matches(getNativeId(this), selector)
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
