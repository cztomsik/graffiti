import { EventTarget } from '../events/EventTarget'
import { Document } from '../nodes/Document'
import { Location } from './Location'
import { History } from './History'
import { requestAnimationFrame, cancelAnimationFrame } from './raf'
import { TODO } from '../util'

const G = globalThis

// note all props will leak to global scope
export class Window extends EventTarget implements globalThis.Window {
  parent = this as any
  window = this.parent
  self = this.window
  history = new History(this, new URL(this.document.URL))
  location = new Location(this.history)
  // TODO
  navigator: any = {
    userAgent: 'graffiti'
  }

  // provided by deno/nodejs and/or polyfilled in worker.ts
  setInterval = G.setInterval
  setTimeout = G.setTimeout
  clearInterval = G.clearInterval
  clearTimeout = G.clearTimeout
  console = G.console
  fetch = G.fetch
  performance = G.performance
  atob = G.atob
  btoa = G.btoa
  queueMicrotask = G.queueMicrotask
  postMessage = G.postMessage
  crypto = G.crypto

  // raf
  requestAnimationFrame = requestAnimationFrame
  cancelAnimationFrame = cancelAnimationFrame

  // react-dom needs both
  HTMLIFrameElement = class {}

  // wouter needs global Event & it could be referenced via window.* too
  Event = Event

  constructor(public readonly document: globalThis.Document) {
    super()
  }

  getComputedStyle(elt: Element, pseudoElt?: string | null): CSSStyleDeclaration {
    // CSSStyleDeclaration
    // - parentRule = null
    // - onChange = NOOP
    // - values = { resolvedProps + layoutProps }
    throw new Error('Method not implemented.')
  }

  // TODOs
  alert = TODO
  blur = TODO
  captureEvents = TODO
  close = TODO
  confirm = TODO
  createImageBitmap = TODO
  departFocus = TODO
  focus = TODO
  getMatchedCSSRules = TODO
  getSelection = TODO
  matchMedia = TODO
  moveBy = TODO
  moveTo = TODO
  open = TODO
  print = TODO
  prompt = TODO
  releaseEvents = TODO
  resizeBy = TODO
  resizeTo = TODO
  scroll = TODO
  scrollBy = TODO
  scrollTo = TODO
  stop = TODO

  get localStorage() {
    return TODO()
  }

  get sessionStorage() {
    return TODO()
  }

  get innerHeight() {
    return TODO()
  }

  get innerWidth() {
    return TODO()
  }

  get outerHeight() {
    return TODO()
  }

  get outerWidth() {
    return TODO()
  }

  get scrollX() {
    return TODO()
  }

  get scrollY() {
    return TODO()
  }

  // ?
  applicationCache
  caches
  clientInformation
  closed
  customElements
  defaultStatus
  devicePixelRatio
  doNotTrack
  event
  external
  frameElement
  frames
  indexedDB
  isSecureContext
  length
  locationbar
  menubar
  name
  offscreenBuffering
  opener
  orientation
  origin
  pageXOffset
  pageYOffset
  personalbar
  screen
  screenLeft
  screenTop
  screenX
  screenY
  scrollbars
  speechSynthesis
  status
  statusbar
  styleMedia
  toolbar
  top
  visualViewport
  [index: number]: globalThis.Window;

  // ignore vendor
  msContentScript
  msWriteProfilerMark
  webkitCancelAnimationFrame
  webkitConvertPointFromNodeToPage
  webkitConvertPointFromPageToNode
  webkitRequestAnimationFrame
}
