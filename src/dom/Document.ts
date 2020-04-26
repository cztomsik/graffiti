import { Node } from './Node'
import { Text } from './Text'
import { Comment } from './Comment'
import { DocumentFragment } from './DocumentFragment'

import { Element } from './Element'
import { HTMLElement } from './HTMLElement'
import { HTMLHtmlElement } from './HTMLHtmlElement'
import { HTMLHeadElement } from './HTMLHeadElement'
import { HTMLBodyElement } from './HTMLBodyElement'
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

type IDocument = globalThis.Document

// too many type errs
export class Document extends Node implements IDocument {
  _nativeId = 0
  _scene = this.defaultView.sceneContext
  _els: HTMLElement[] = []

  // title is only defined here, not on the window itself
  title = ''

  documentElement = this.createElement('html')
  head = this.createElement('head')
  body = this.createElement('body')

  activeElement
  // last time over for enter/leave
  _overElement
  // mousedown origin
  _clickedElement

  constructor(public defaultView) {
    super(null, Node.DOCUMENT_NODE)

    // special setup
    this.childNodes.push(this.documentElement)
    ;(this.documentElement as any).parentNode = this
    this.documentElement.appendChild(this.head)
    this.documentElement.appendChild(this.body)
  }

  get location() {
    return this.defaultView.location
  }

  createElement(tagName: string): HTMLElement {
    // happy-case
    // - simple comparison of interned strings
    // - ordered by likelihood
    //
    // note we are setting correct (interned) tagName here
    // it could be in each impl but it would be one extra contructor call
    switch (tagName) {
      case 'div': return new HTMLDivElement(this, 'DIV')
      case 'span': return new HTMLSpanElement(this, 'SPAN')
      case 'a': return new HTMLAnchorElement(this, 'A')
      case 'button': return new HTMLButtonElement(this, 'BUTTON')
      case 'input': return new HTMLInputElement(this, 'INPUT')
      case 'textarea': return new HTMLTextAreaElement(this, 'TEXTAREA')
      case 'table': return new HTMLTableElement(this, 'TABLE')
      case 'thead': return new HTMLTableSectionElement(this, 'THEAD')
      case 'tbody': return new HTMLTableSectionElement(this, 'TBODY')
      case 'tr': return new HTMLTableRowElement(this, 'TR')
      case 'td': return new HTMLTableCellElement(this, 'TD')
      case 'th': return new HTMLTableHeaderCellElement(this, 'TH')
      case 'head': return new HTMLHeadElement(this, 'HEAD')
      case 'body': return new HTMLBodyElement(this, 'BODY')
      // special-case
      case 'html': return new HTMLHtmlElement(this, 'HTML')

      default:
        const lower = tagName.toLowerCase()

        if (tagName === lower) {
          return new HTMLUnknownElement(this, tagName.toUpperCase())
        }

        return this.createElement(lower as any)
    }
  }

  createElementNS(ns: string | null, tagName: string, options?): any {
    switch (ns) {
      case 'http://www.w3.org/2000/svg':
        switch (tagName) {
          case 'svg': return new SVGSVGElement(this, 'svg')
          case 'g': return new SVGGElement(this, 'g')
          default: return new SVGElement(this, tagName)
        }

      default:
        return new Element(this, tagName )
    }
  }

  createTextNode(data: string): Text {
    return new Text(this, data)
  }

  createComment(data: string): Comment {
    const c = new Comment(this, data)

    // empty text node for now
    // (temporary hack)
    c._nativeId = this._scene.createText()

    return c
  }

  createDocumentFragment(): DocumentFragment {
    return new DocumentFragment(this)
  }

  getElementById(id): HTMLElement | null {
    // return this.querySelector(`#${id}`)
    return this._els.find(el => el.id === id)
  }

  querySelector(selectors: string) {
    return this.documentElement.querySelector(selectors)
  }

  querySelectorAll(selectors: string) {
    return this.documentElement.querySelectorAll(selectors)
  }

  _getEl(_nativeId) {
    return this._els[_nativeId]
  }

  _getTheParent() {
    return this.defaultView
  }

  get forms() { return this.getElementsByTagName('form') }
  get images() { return this.getElementsByTagName('img') }
  get links() { return this.getElementsByTagName('link') }
  get scripts() { return this.getElementsByTagName('script') }

  // later
  adoptNode
  alinkColor
  all
  anchors
  applets
  bgColor
  captureEvents
  caretPositionFromPoint
  caretRangeFromPoint
  characterSet
  charset
  clear
  close
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
  fgColor
  fullscreen
  fullscreenElement
  fullscreenEnabled
  getAnimations
  getElementsByName
  getSelection
  hasFocus
  hidden
  implementation
  importNode
  inputEncoding
  lastModified
  linkColor
  open
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
  styleSheets
  timeline
  URL
  visibilityState
  vlinkColor
  write
  writeln
}
