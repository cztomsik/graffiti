import { Node } from './Node'
import { NodeList } from './NodeList'
import { Text } from './Text'
import { Comment } from './Comment'
import { DocumentFragment } from './DocumentFragment'
import { StyleSheetList } from '../css/StyleSheetList'
import { UNSUPPORTED } from '../util'

import { HTMLHtmlElement } from './HTMLHtmlElement'
import { HTMLHeadElement } from './HTMLHeadElement'
import { HTMLBodyElement } from './HTMLBodyElement'
import { HTMLStyleElement } from './HTMLStyleElement'
import { HTMLScriptElement } from './HTMLScriptElement'
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
import { DOMImplementation } from '../dom/DOMImplementation'
//import { nwsapi } from '../nwsapi'

export class Document extends Node implements globalThis.Document {
  readonly ownerDocument
  readonly defaultView: Window & typeof globalThis | null = null
  readonly implementation = new DOMImplementation()
  readonly childNodes = new NodeList<ChildNode>()
  readonly compatMode = 'CSS1Compat'
  //readonly nwsapi = nwsapi({ document: this })

  // TODO: getter, should be last focused or body (which can be null sometimes)
  readonly activeElement: Element | null = null
  // last time over for enter/leave
  _overElement
  // mousedown origin
  _clickedElement

  constructor(private readonly _adapter = NOOP_ADAPTER) {
    super(null as any)

    // non-standard, we are using parent.ownerDocument in Node.insertBefore()
    // to dispatch changes (and we are using doc.appendChild() during parsing)
    // if it's a problem we could use child.ownerDocument (should be same)
    this.ownerDocument = this
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

  get title() {
    return this.head?.childNodes.find(n => n.localName === 'title')?.data ?? ''
  }

  set title(title) {
    const head = this.head || this.appendChild(this.createElement('head'))
    const titleEl = head.childNodes.find(n => n.localName === 'title') ?? head.appendChild(this.createElement('title'))

    titleEl.data = title
  }

  get location() {
    // DOMParser docs should have null (TS is wrong)
    return (this.defaultView?.location ?? null) as any
  }

  // TODO: basic custom elements (no shadow DOM)
  createElement(tagName: string, options?) {
    // happy-case
    // - tagName in lowercase means it's also localName
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
      case 'script': return new HTMLScriptElement(this, tagName)
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

  get isConnected(): boolean {
    return true
  }

  _getTheParent() {
    return this.defaultView
  }

  get styleSheets(): StyleSheetList {
    // TODO: add default style sheet
    // TODO: get [SHEET_SYMBOL] and create/remove that in adapter
    return new StyleSheetList(this.querySelectorAll('style').map(s => undefined/*s.sheet*/))
  }

  get forms() { return this.getElementsByTagName('form') }
  get images() { return this.getElementsByTagName('img') }
  get links() { return this.getElementsByTagName('link') }
  get scripts() { return this.getElementsByTagName('script') }

  // TODO: querySelector, nwsapi
  getElementById(id) {
    return document.body.childNodes.find(n => n.id === id)
  }

  // change notifiers
  _childInserted(parent, child, index) {
    this._adapter.childInserted(parent, child, index)
  }

  _childRemoved(parent, child) {
    this._adapter.childRemoved(parent, child)
  }

  _styleChanged(el, prop, value) {
    this._adapter.styleChanged(el, prop, value)
  }

  _dataChanged(cdata, data) {
    this._adapter.dataChanged(cdata, data)
  }

  // intentionally left out (out-of-scope)
  clear = UNSUPPORTED
  close = UNSUPPORTED
  open = UNSUPPORTED
  write = UNSUPPORTED
  writeln = UNSUPPORTED
  adoptNode = UNSUPPORTED
  importNode = UNSUPPORTED
  createAttribute = UNSUPPORTED
  createAttributeNS = UNSUPPORTED

  // maybe later
  caretPositionFromPoint
  characterSet
  charset
  contentType
  cookie
  createCDATASection
  createEvent
  createExpression
  createNodeIterator
  createNSResolver
  createProcessingInstruction
  createRange
  createTreeWalker
  currentScript
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
  fullscreenElement
  fullscreenEnabled
  getAnimations
  getElementsByName
  getElementsByTagName
  getElementsByTagNameNS
  getElementsByClassName
  getSelection
  hidden
  inputEncoding
  lastModified
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
  scrollingElement
  timeline
  URL
  visibilityState

  // deprecated
  alinkColor
  all
  anchors
  applets
  bgColor
  fgColor
  fullscreen
  linkColor
  captureEvents
  caretRangeFromPoint
  releaseEvents
  vlinkColor
}

export type DocumentAdapter = typeof NOOP_ADAPTER

const NOOP_ADAPTER = {
  childInserted: (parent, child, index) => {},
  childRemoved: (parent, child) => {},
  styleChanged: (el, prop, value) => {},
  dataChanged: (cdata, data) => {}
}

type Doc = Document

declare global {
  interface Document extends Doc {}
}
