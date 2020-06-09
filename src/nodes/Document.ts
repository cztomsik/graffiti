import assert from 'assert'

import { NodeList } from './NodeList'
import { Node } from './Node'
import { Text } from './Text'
import { Comment } from './Comment'
import { DocumentFragment } from './DocumentFragment'

import { HTMLHtmlElement } from './HTMLHtmlElement'
import { HTMLHeadElement } from './HTMLHeadElement'
import { HTMLBodyElement } from './HTMLBodyElement'
import { HTMLStyleElement } from './HTMLStyleElement'
import { HTMLDivElement } from './HTMLDivElement'
import { HTMLSpanElement } from './HTMLSpanElement'
import { HTMLInputElement } from './HTMLInputElement'
import { HTMLTextAreaElement } from './HTMLTextAreaElement'
import { HTMLButtonElement } from './HTMLButtonElement'
import { HTMLUnknownElement } from './HTMLUnknownElement'
import { HTMLAnchorElement } from './HTMLAnchorElement'
import { SVGElement } from './SVGElement'
import { SVGSVGElement } from './SVGSvgElement'
import { SVGGElement } from './SVGGElement'
import { HTMLTableSectionElement } from './HTMLTableSectionElement'
import { HTMLTableElement } from './HTMLTableElement'
import { HTMLTableCellElement } from './HTMLTableCellElement'
import { HTMLTableHeaderCellElement } from './HTMLTableHeaderCellElement'
import { HTMLTableRowElement } from './HTMLTableRowElement'

import { StyleSheetList } from '../css/StyleSheetList'

export class Document extends Node implements globalThis.Document {
  readonly childNodes = new NodeList<ChildNode>()
  // TODO: add default style sheet
  readonly styleSheets = new StyleSheetList();

  // title is only defined here, not on the window itself
  title = ''

  // TODO: getter, should be last focused or body (which can be null sometimes)
  readonly activeElement: Element | null = null
  // last time over for enter/leave
  _overElement
  // mousedown origin
  _clickedElement

  constructor(public readonly defaultView, private _native) {
    super(null)

    this._native.initDocument(this)

    const html = this.createElement('html')
    html.appendChild(this.createElement('head'))
    html.appendChild(this.createElement('body'))

    this.appendChild(html)
  }

  get nodeType() {
    return Node.DOCUMENT_NODE
  }

  get nodeName() {
    return '#document'
  }

  get documentElement() {
    // chrome allows removing root & appending a new one
    return this.childNodes[0] ?? null
  }

  get head() {
    return this.documentElement?.childNodes.find(n => n.localName === 'head') ?? null
  }

  get body() {
    return this.documentElement?.childNodes.find(n => n.localName === 'body') ?? null
  }

  get location() {
    return this.defaultView.location
  }

  // TODO: basic custom elements (no shadow DOM)
  createElement(tagName: string, options?) {
    // happy-case
    // - simple comparison of interned strings
    // - ordered by likelihood
    switch (tagName) {
      case 'div': return new HTMLDivElement(this, tagName)
      case 'span': return new HTMLSpanElement(this, tagName)
      case 'a': return new HTMLAnchorElement(this, tagName)
      case 'button': return new HTMLButtonElement(this, tagName)
      case 'input': return new HTMLInputElement(this, tagName)
      case 'textarea': return new HTMLTextAreaElement(this, tagName)
      case 'table': return new HTMLTableElement(this, tagName)
      case 'thead': return new HTMLTableSectionElement(this, tagName)
      case 'tbody': return new HTMLTableSectionElement(this, tagName)
      case 'tr': return new HTMLTableRowElement(this, tagName)
      case 'td': return new HTMLTableCellElement(this, tagName)
      case 'th': return new HTMLTableHeaderCellElement(this, tagName)
      case 'style': return new HTMLStyleElement(this, tagName)
      case 'head': return new HTMLHeadElement(this, tagName)
      case 'body': return new HTMLBodyElement(this, tagName)
      case 'html': return new HTMLHtmlElement(this, tagName)

      // otherwise try lowercase and eventually fall-back to HTMLUnknownElement
      default:
        const lower = tagName.toLowerCase()

        if (tagName === lower) {
          return new HTMLUnknownElement(this, tagName)
        }

        return this.createElement(lower as any)
    }
  }

  createElementNS(ns: string | null, tagName: string, options?): any {
    switch (ns) {
      case 'http://www.w3.org/2000/svg':
        switch (tagName) {
          case 'svg': return new SVGSVGElement(this, tagName)
          case 'g': return new SVGGElement(this, tagName)
          default: return new SVGElement(this, tagName)
        }

      default:
        return new HTMLUnknownElement(this, tagName)
    }
  }

  createTextNode(data: string): Text {
    return new Text(data, this)
  }

  createComment(data: string): Comment {
    // empty text node for now
    // (temporary hack)
    return new Comment(data, this)
  }

  createDocumentFragment(): DocumentFragment {
    return new DocumentFragment(this)
  }

  hasFocus(): boolean {
    // TODO: not sure if it shouldn't also be !== body
    return !!this.activeElement
  }

  _insertChildAt(child, index) {
    assert(index === 0, 'only one root is allowed')
    assert(child.nodeType === Node.ELEMENT_NODE, 'only element can be root')

    super._insertChildAt(child, index)

    this._native.setRoot(this, child)
  }

  querySelector(selectors: string, element?) {
    return this._native.querySelector(this, selectors, element)
  }

  querySelectorAll(selectors: string, element?) {
    return this._native.querySelectorAll(this, selectors, element)
  }

  get isConnected(): boolean {
    return true
  }

  _getTheParent() {
    return this.defaultView
  }

  get forms() { return this.getElementsByTagName('form') }
  get images() { return this.getElementsByTagName('img') }
  get links() { return this.getElementsByTagName('link') }
  get scripts() { return this.getElementsByTagName('script') }

  // native
  _initElement(el, localName) {
    this._native.initElement(this, el, localName)
  }

  _elChildInserted(el, child, index) {
    this._native.insertChildAt(this, el, child, index)
  }

  _elChildRemoved(el, child) {
    this._native.removeChild(this, el, child)
  }

  _initTextNode(textNode, data) {
    this._native.initTextNode(this, textNode, data)
  }

  _textUpdated(textNode, text) {
    this._native.setText(this, textNode, text)
  }

  // intentionally left out (TODO: UNSUPPORTED())
  all
  clear
  close
  currentScript
  open
  write
  writeln

  // maybe later
  adoptNode
  alinkColor
  anchors
  applets
  bgColor
  captureEvents
  caretPositionFromPoint
  caretRangeFromPoint
  characterSet
  charset
  compatMode
  contentType
  cookie
  createAttribute
  createAttributeNS
  createCDATASection
  createEvent
  createExpression
  createNodeIterator
  createNSResolver
  createProcessingInstruction
  createRange
  createTreeWalker
  designMode
  dir
  doctype
  documentURI
  domain
  elementFromPoint
  elementsFromPoint
  embeds
  evaluate
  execCommand
  exitFullscreen
  exitPointerLock
  fgColor
  fullscreen
  fullscreenElement
  fullscreenEnabled
  getAnimations
  getElementById
  getElementsByName
  getElementsByTagName
  getElementsByTagNameNS
  getElementsByClassName
  getSelection
  hidden
  implementation
  importNode
  inputEncoding
  lastModified
  linkColor
  origin
  plugins
  pointerLockElement
  queryCommandEnabled
  queryCommandIndeterm
  queryCommandState
  queryCommandSupported
  queryCommandValue
  readyState
  referrer
  releaseEvents
  scrollingElement
  timeline
  URL
  visibilityState
  vlinkColor
}
