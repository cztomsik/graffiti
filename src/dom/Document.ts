import { Node } from './Node'
import { send } from '../native'
import {
  NodeList,
  Text,
  Comment,
  DocumentFragment,
  HTMLHtmlElement,
  HTMLHeadElement,
  HTMLLinkElement,
  HTMLBodyElement,
  HTMLStyleElement,
  HTMLScriptElement,
  HTMLDivElement,
  HTMLSpanElement,
  HTMLInputElement,
  HTMLIFrameElement,
  HTMLTextAreaElement,
  HTMLButtonElement,
  HTMLUnknownElement,
  HTMLAnchorElement,
  SVGElement,
  SVGSVGElement,
  SVGGElement,
  HTMLTableSectionElement,
  HTMLTableElement,
  HTMLTableCellElement,
  HTMLTableHeaderCellElement,
  HTMLTableRowElement,
  DOMImplementation,
} from './index'

import { StyleSheetList } from '../css/StyleSheetList'
import { UNSUPPORTED } from '../util'
import { Window } from '../window/Window'
import { IDocument } from '../types'

import { Event } from '../events/Event'

export class Document extends Node implements IDocument {
  readonly ownerDocument
  readonly implementation = new DOMImplementation()
  readonly childNodes = new NodeList<ChildNode>()

  constructor(public readonly defaultView: (Window & typeof globalThis) | null = null) {
    // TS defines Node.ownerDocument as nullable but redefines it on every subclass except Document
    super(null as any)

    // non-standard, we are using parent.ownerDocument in Node.insertBefore()
    // and we are using doc.appendChild() during parsing
    // if it's ever a problem we could use child.ownerDocument
    this.ownerDocument = this

    const docId = send('CreateDocument')
    const sendDocMsg = msg => send({ DocumentMsg: [docId, msg] })

    const WEAK_REFS = Symbol()
    this[WEAK_REFS] = new Map()
    const lookup = id => (id && this[WEAK_REFS].get(id)?.deref()) ?? null

    const ID = Symbol()
    const initNode = (node, id) => {
      node[ID] = id
      this[WEAK_REFS].set(id, new WeakRef(node))
      // TODO: register for drop
    }

    this[initText] = (node, text) => initNode(node, sendDocMsg({ CreateTextNode: text }))
    this[setText] = (node, text) => sendDocMsg({ SetText: [node[ID], text] })
    this[initElement] = (el, localName) => initNode(el, sendDocMsg({ CreateElement: localName }))
    this[setAttribute] = (el, k, v) => sendDocMsg({ SetAttribute: [el[ID], k, v] })
    this[removeAttribute] = (el, k) => sendDocMsg({ RemoveAttribute: [el[ID], k] })
    this[appendChild] = (parent, child) => sendDocMsg({ AppendChild: [parent[ID], child[ID]] })
    this[insertBefore] = (parent, child, before) => sendDocMsg({ InsertBefore: [parent[ID], child[ID], before[ID]] })
    this[removeChild] = (parent, child) => sendDocMsg({ RemoveChild: [parent[ID], child[ID]] })
    this[elementMatches] = (node, sel) => sendDocMsg({ ElementMatches: [node[ID], sel] })
    this[querySelector] = (node, sel) => lookup(sendDocMsg({ QuerySelector: [node[ID], sel] }))
    this[querySelectorAll] = (node, sel) => sendDocMsg({ QuerySelectorAll: [node[ID], sel] })?.map(lookup) ?? []

    initNode(this, 1)
  }

  get nodeType() {
    return Node.DOCUMENT_NODE
  }

  get nodeName() {
    return '#document'
  }

  get compatMode() {
    return 'CSS1Compat'
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
    const head = this.head ?? this.appendChild(this.createElement('head'))
    const titleEl = head.childNodes.find(n => n.localName === 'title') ?? head.appendChild(this.createElement('title'))

    titleEl.data = title
  }

  get location() {
    // DOMParser-created documents should have null location (TS is wrong)
    return this.defaultView?.location ?? (null as any)
  }

  // TODO: this is wrong and it should rather use lookup table
  //       because string comparison is not O(1)
  //
  // TODO: basic custom elements (no shadow DOM)
  // prettier-ignore
  createElement(tagName: string, options?) {
    // happy-case
    // - tagName in lowercase means it's also localName
    // - simple comparison of interned strings
    // - ordered by likelihood
    switch (tagName) {
      case 'div': return new HTMLDivElement(tagName, this)
      case 'span': return new HTMLSpanElement(tagName, this)
      case 'a': return new HTMLAnchorElement(tagName, this)
      case 'button': return new HTMLButtonElement(tagName, this)
      case 'input': return new HTMLInputElement(tagName, this)
      case 'textarea': return new HTMLTextAreaElement(tagName, this)
      case 'table': return new HTMLTableElement(tagName, this)
      case 'thead': return new HTMLTableSectionElement(tagName, this)
      case 'tbody': return new HTMLTableSectionElement(tagName, this)
      case 'tr': return new HTMLTableRowElement(tagName, this)
      case 'td': return new HTMLTableCellElement(tagName, this)
      case 'th': return new HTMLTableHeaderCellElement(tagName, this)
      case 'style': return new HTMLStyleElement(tagName, this)
      case 'script': return new HTMLScriptElement(tagName, this)
      //case 'canvas': return new HTMLCanvasElement(tagName, this)
      case 'link': return new HTMLLinkElement(tagName, this)
      case 'iframe': return new HTMLIFrameElement(tagName, this)
      case 'head': return new HTMLHeadElement(tagName, this)
      case 'body': return new HTMLBodyElement(tagName, this)
      case 'html': return new HTMLHtmlElement(tagName, this)

      // otherwise try lowercase and eventually fall-back to HTMLUnknownElement
      default:
        if (typeof tagName !== 'string') {
          tagName = '' + tagName
        }

        const lower = tagName.toLowerCase()

        if (tagName === lower) {
          return new HTMLUnknownElement(tagName, this)
        }

        return this.createElement(lower as any)
    }
  }

  // prettier-ignore
  createElementNS(ns: string | null, tagName: string, options?): any {
    switch (ns) {
      case 'http://www.w3.org/2000/svg':
        switch (tagName) {
          case 'svg': return new SVGSVGElement(tagName, this)
          case 'g': return new SVGGElement(tagName, this)
          default: return new SVGElement(tagName, this)
        }

      default:
        return new HTMLUnknownElement(tagName, this)
    }
  }

  createTextNode(data: string): Text {
    return new Text(data, this)
  }

  createComment(data: string): Comment {
    return new Comment(data, this)
  }

  createDocumentFragment(): DocumentFragment {
    return new DocumentFragment(this)
  }

  elementFromPoint(x, y): Element | null {
    console.log('TODO: doc.elementFromPoint()')
    return null
  }

  hasFocus(): boolean {
    // TODO: not sure if it shouldn't also be !== body
    return !!this.activeElement
  }

  get activeElement() {
    console.log('TODO: activeElement')
    return null
  }

  get isConnected(): boolean {
    return true
  }

  get styleSheets(): StyleSheetList {
    return new StyleSheetList(this.getElementsByTagName('style').map(s => s.sheet))
  }

  get forms() {
    return this.getElementsByTagName('form')
  }

  get images() {
    return this.getElementsByTagName('img')
  }

  get links() {
    return this.getElementsByTagName('link')
  }

  get scripts() {
    return this.getElementsByTagName('script')
  }

  // deprecated
  createEvent(type) {
    // TODO: return appropriate subclass
    // TODO: bubbles/cancelable
    return new Event(type.toLowerCase()) as any
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
  elementsFromPoint
  embeds
  evaluate
  execCommand
  exitFullscreen
  exitPictureInPicture
  exitPointerLock
  fonts
  fullscreenElement
  fullscreenEnabled
  getAnimations
  getElementsByName
  getSelection
  hasStorageAccess
  hidden
  inputEncoding
  lastModified
  origin
  pictureInPictureElement
  pictureInPictureEnabled
  plugins
  pointerLockElement
  queryCommandEnabled
  queryCommandIndeterm
  queryCommandState
  queryCommandSupported
  queryCommandValue
  readyState
  referrer
  requestStorageAccess
  rootElement
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

// package-private
export const initText = Symbol()
export const setText = Symbol()
export const initElement = Symbol()
export const setAttribute = Symbol()
export const removeAttribute = Symbol()
// export const setStyleProp = Symbol();
export const appendChild = Symbol()
export const insertBefore = Symbol()
export const removeChild = Symbol()
export const elementMatches = Symbol()
export const querySelector = Symbol()
export const querySelectorAll = Symbol()

type Doc = Document

declare global {
  interface Document extends Doc {}
}
