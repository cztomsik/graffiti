import * as globals from './globals'
import { EventTarget } from '../events/index'
import { Document } from '../dom/index'
import { Location } from './Location'
import { History } from './History'
import { Storage } from './Storage'
import { requestAnimationFrame, cancelAnimationFrame } from './raf'
import { NOOP, TODO, fetch } from '../util'
import { send } from '../native'

export const VIEWPORT_ID = Symbol()

// note all props will leak to global scope
export class Window extends EventTarget implements globalThis.Window {
  #document = new Document()
  #history = new History(this)
  #location = new Location(this.#history)
  #sessionStorage = globalThis.sessionStorage ?? new Storage()
  #localStorage = globalThis.localStorage ?? new Storage()

  // TODO
  navigator: any = {
    userAgent: 'graffiti',
  }

  // TODO (vite needs it)
  customElements = { define: NOOP, get: NOOP } as any

  // provided by deno/nodejs
  fetch = fetch
  setInterval = setInterval
  setTimeout = setTimeout
  clearInterval = clearInterval
  clearTimeout = clearTimeout
  console = console
  performance = performance
  atob = atob
  btoa = btoa
  queueMicrotask = queueMicrotask
  postMessage = postMessage
  crypto = globalThis.crypto
  reportError = globalThis.reportError
  structuredClone = globalThis.structuredClone

  // raf
  requestAnimationFrame = requestAnimationFrame
  cancelAnimationFrame = cancelAnimationFrame

  constructor() {
    super()

    this[VIEWPORT_ID] = send({ CreateViewport: [[1024, 768], this.document[DOC_ID]] })

    Object.assign(this, globals)
  }

  get window() {
    return this as any
  }

  get self() {
    return this as any
  }

  get top() {
    return this as any
  }

  get opener() {
    return null
  }

  get parent() {
    return this as any
  }

  get frameElement() {
    return null
  }

  get document() {
    return this.#document
  }

  get history() {
    return this.#history
  }

  get location() {
    return this.#location
  }

  set location(url: any) {
    this.#location.assign(url)
  }

  get sessionStorage() {
    return this.#sessionStorage
  }

  get localStorage() {
    return this.#localStorage
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
  moveBy = (x, y) => this.moveTo(this.screenX + x, this.screenY + y)
  moveTo = (x, y) => console.log('TODO: window.moveTo()')
  resizeBy = () => this.resizeTo(this.outerWidth, this.outerHeight)
  resizeTo = (outerWidth, outerHeight) => console.log('TODO: window.resizeTo()')

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
  stop = NOOP

  scroll = (...args) => this.document.documentElement.scroll(...args)
  scrollBy = (...args) => this.document.documentElement.scroll(...args)
  scrollTo = (...args) => this.document.documentElement.scroll(...args)

  get innerHeight() {
    console.log('TODO: window.innerHeight')
    return 768
  }

  get innerWidth() {
    console.log('TODO: window.innerWidth')
    return 1024
  }

  get screenX(): number {
    console.log('TODO: screenX')
    return 0
  }

  get screenY(): number {
    console.log('TODO: screenY')
    return 0
  }

  get screenLeft() {
    return this.screenX
  }

  get screenTop() {
    return this.screenY
  }

  get outerHeight() {
    console.log('TODO: window.outerHeight')
    return 768
  }

  get outerWidth() {
    console.log('TODO: window.outerWidth')
    return 1024
  }

  get pageXOffset() {
    return this.document.documentElement.scrollLeft
  }

  get pageYOffset() {
    return this.document.documentElement.scrollTop
  }

  get scrollX() {
    return this.pageXOffset
  }

  get scrollY() {
    return this.pageYOffset
  }

  get menubar() {
    return { visible: false }
  }

  get locationbar() {
    return { visible: false }
  }

  get personalbar() {
    return { visible: false }
  }

  get statusbar() {
    return { visible: false }
  }

  get toolbar() {
    return { visible: false }
  }

  get scrollbars() {
    return { visible: false }
  }

  // deprecated
  captureEvents
  event
  external
  orientation
  releaseEvents

  // ?
  applicationCache
  caches
  cancelIdleCallback
  clientInformation
  closed
  crossOriginIsolated
  defaultStatus
  departFocus
  devicePixelRatio
  doNotTrack
  frames
  indexedDB
  isSecureContext
  length
  name
  offscreenBuffering
  origin
  requestIdleCallback
  screen
  speechSynthesis
  status
  styleMedia
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

// attempt to install window instance into already existing globalThis
export const makeGlobal = window => {
  // save before deletion
  const log = console.log.bind(console)

  // cleanup
  for (const k of [
    ...Object.getOwnPropertyNames(window),
    ...Object.getOwnPropertyNames(Window.prototype),
    ...Object.getOwnPropertyNames(EventTarget.prototype),
  ]) {
    if (!Reflect.deleteProperty(globalThis, k)) {
      log(`couldnt delete global ${k}`)
    }
  }

  // TODO: this could be enough (except globalThis !== window)
  //       but maybe Window() constructor could accept globalThis and return it then
  //       or maybe we could set a Symbol or WeakMap, anything
  //       I am not sure yet but this is probably ok for now
  //
  // we could use VM context for node but I don't know what to use for deno
  // it has to be proxy because Window is using private class fields
  Object.setPrototypeOf(
    globalThis,
    new Proxy(window, {
      // I think we only need these two (in order to fix private fields)
      get(target, prop, _receiver) {
        return Reflect.get(target, prop)
      },
      set(target, prop, value, _receiver) {
        return Reflect.set(target, prop, value)
      },
    })
  )
}
