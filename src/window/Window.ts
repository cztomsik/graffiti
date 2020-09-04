import { EventTarget } from '../events/EventTarget'
import { Document } from '../nodes/Document'
import { Location } from './Location'
import { History } from './History'
import { TODO } from '../util'

// chrome/firefox has it this way too
class WindowProperties extends EventTarget {}

export class Window extends WindowProperties implements globalThis.Window {
  window = this as any
  self = this.window
  history = new History(this, new URL(this.document.URL))
  location = new Location(this.history)
  // TODO
  navigator: any = {
    userAgent: 'graffiti'
  }

  // forward globals
  setInterval = setInterval
  setTimeout = setTimeout
  clearInterval = clearInterval
  clearTimeout = clearTimeout
  console = console
  fetch = fetch
  performance = performance || globalThis.require('performance').performance
  // TODO: quickjs
  // ? https://github.com/jsdom/abab
  atob = atob || (str => new Buffer(str, 'base64').toString('binary'))
  btoa = btoa || (str => Buffer.from(str).toString('base64'))

  constructor(public readonly document: globalThis.Document) {
    super()
  }

  // TODOs
  alert = TODO
  blur = TODO
  cancelAnimationFrame = TODO
  captureEvents = TODO
  close = TODO
  confirm = TODO
  createImageBitmap = TODO
  departFocus = TODO
  focus = TODO
  getComputedStyle = TODO
  getMatchedCSSRules = TODO
  getSelection = TODO
  matchMedia = TODO
  moveBy = TODO
  moveTo = TODO
  open = TODO
  postMessage = TODO
  print = TODO
  prompt = TODO
  queueMicrotask = TODO
  releaseEvents = TODO
  requestAnimationFrame = TODO
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

  get crypto() {
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
  parent
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



/*

import { performance } from 'perf_hooks'

import { Event } from '../events/Event'
import { handleWindowEvent } from '../events/handleWindowEvent'

import { NOOP } from '../util'

// beware, all props/meths (incl. privates) are in global scope
export class Window extends EventTarget {
  _nextAnimHandle = 1
  _animCbs: FrameRequestCallback[] = []

  constructor(private readonly _native) {
    super()
  }

  requestAnimationFrame(callback: FrameRequestCallback): number {
    if (this._animCbs.length === 0) {
      const animate = () => {
        const timestamp = performance.now()

        for (const cb of this._animCbs) {
          cb(timestamp)
        }

        this._animCbs = []
      }

      setImmediate(animate)
    }

    this._animCbs.push(callback)

    return this._nextAnimHandle++
  }

  cancelAnimationFrame(handle: number) {
    const index = this._nextAnimHandle - handle

    if (index >= 0) {
      // replace so that other indices remain valid too
      this._animCbs[index] = NOOP
    }
  }
}


/*

export class Window extends EventTarget implements globalThis.Window {
  // react-dom needs both
  HTMLIFrameElement = class {}

  // wouter needs global Event & it could be referenced via window.* too
  Event = Event

  _handleEvent(event) {
    handleWindowEvent(this.document, event)
  }

}
*/
