import * as globals from './globals'
import { EventTarget } from '../events/index'
import { Document } from '../dom/index'
import { Location } from './Location'
import { History } from './History'
import { Storage } from './Storage'
import { requestAnimationFrame, cancelAnimationFrame } from './raf'
import { NOOP, TODO, fetch } from '../util'

const G = globalThis

// note all props will leak to global scope
export class Window extends EventTarget implements globalThis.Window {
  readonly document = new Document()
  parent = this as any
  window = this.parent
  self = this.window
  history = new History(this)
  location = new Location(this.history)
  sessionStorage = new Storage()
  localStorage = new Storage()
  // TODO
  navigator: any = {
    userAgent: 'graffiti',
  }
  // TODO (vite needs it)
  customElements = { define: NOOP } as any
  fetch = fetch

  // TODO (wpt, autobind global fns?)
  addEventListener = this.addEventListener.bind(this)
  removeEventListener = this.removeEventListener.bind(this)

  // provided by deno/nodejs
  setInterval = G.setInterval
  setTimeout = G.setTimeout
  clearInterval = G.clearInterval
  clearTimeout = G.clearTimeout
  console = G.console
  performance = G.performance
  atob = G.atob
  btoa = G.btoa
  queueMicrotask = G.queueMicrotask
  postMessage = G.postMessage
  crypto = G.crypto

  // raf
  requestAnimationFrame = requestAnimationFrame
  cancelAnimationFrame = cancelAnimationFrame

  constructor() {
    super()

    Object.assign(this, globals)
  }

  getComputedStyle(elt: Element, pseudoElt?: string | null): CSSStyleDeclaration {
    // CSSStyleDeclaration
    // - parentRule = null
    // - onChange = NOOP
    // - values = { resolvedProps + layoutProps }
    throw new Error('Method not implemented.')
  }

  // TODO (and no-op in <iframe>)
  blur = () => console.log('TODO: window.blur()')
  focus = () => console.log('TODO: window.focus()')
  moveBy = () => console.log('TODO: window.moveBy()')
  moveTo = () => console.log('TODO: window.moveTo()')
  resizeBy = () => console.log('TODO: window.resizeBy()')
  resizeTo = () => console.log('TODO: window.resizeTo()')

  // TODOs
  alert = () => console.log('TODO: window.alert()')
  close = () => console.log('TODO: window.close()')
  confirm = () => TODO()
  createImageBitmap = () => TODO()
  getMatchedCSSRules = () => TODO()
  getSelection = () => TODO()
  matchMedia = () => ({ matches: false } as any)
  open = () => TODO()
  print = () => TODO()
  prompt = () => TODO()
  scroll = () => console.log('TODO: window.scroll()')
  scrollBy = () => console.log('TODO: window.scrollBy()')
  scrollTo = () => console.log('TODO: window.scrollTo()')
  stop = NOOP

  get innerHeight() {
    console.log('TODO: window.innerHeight')
    return 768
  }

  get innerWidth() {
    console.log('TODO: window.innerWidth')
    return 1024
  }

  get outerHeight() {
    console.log('TODO: window.outerHeight')
    return 768
  }

  get outerWidth() {
    console.log('TODO: window.outerWidth')
    return 1024
  }

  get scrollX() {
    console.log('TODO: window.scrollX')
    return 0
  }

  get scrollY() {
    console.log('TODO: window.scrollY')
    return 0
  }

  // deprecated
  captureEvents
  releaseEvents

  // ?
  applicationCache
  caches
  clientInformation
  closed
  defaultStatus
  departFocus
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
  visualViewport;
  [index: number]: globalThis.Window

  // ignore vendor
  msContentScript
  msWriteProfilerMark
  webkitCancelAnimationFrame
  webkitConvertPointFromNodeToPage
  webkitConvertPointFromPageToNode
  webkitRequestAnimationFrame
}

export const makeGlobal = window => {
  // cleanup
  for (const k of Object.keys(window)) {
    if (!Reflect.deleteProperty(globalThis, k)) {
      //console.log(`couldnt delete global ${k}`)
    }
  }

  // TODO: this is probably not enough
  // we could use VM context for node but I don't know what to use for deno
  Object.setPrototypeOf(globalThis, window)
}
