import vm from 'vm'

import { Document } from './Document'
import { Event } from '../events/Event'
import { EventTarget } from '../events/EventTarget'
import { handleWindowEvent } from '../events/handleWindowEvent'
import { Location } from './Location'
import { History } from './History'

// BTW: in chrome, it's Window extends WindowProperties extends EventTarget

export class Window extends EventTarget implements globalThis.Window {
  window: any = this
  self: any = this

  document = new Document(this)

  // minimal impl for mithril/wouter
  history = new History(this)
  location = new Location(this.history)

  _context = vm.createContext(this)

  eval(code) {
    return vm.runInContext(code, this._context)
  }

  // react-dom needs both
  navigator: any = {
    userAgent: 'graffiti'
  }
  HTMLIFrameElement = class {}

  // wouter needs global Event & it could be referenced via window.* too
  Event = Event

  constructor(private id) {
    super()
  }

  _handleEvent(event) {
    handleWindowEvent(this.document, event)
  }

  atob = str => new Buffer(str, 'base64').toString('binary')
  btoa = str => Buffer.from(str).toString('base64')

  // forward globals
  setInterval = setInterval
  setTimeout = setTimeout
  clearInterval = clearInterval
  clearTimeout = clearTimeout
  console = console

  // maybe later
  alert
  applicationCache
  blur
  caches
  cancelAnimationFrame
  captureEvents
  clientInformation
  close
  closed
  confirm
  createImageBitmap
  crypto
  customElements
  defaultStatus
  departFocus
  devicePixelRatio
  doNotTrack
  event
  external
  fetch
  focus
  frameElement
  frames
  getComputedStyle
  getMatchedCSSRules
  getSelection
  indexedDB
  innerHeight
  innerWidth
  isSecureContext
  length
  localStorage
  locationbar
  matchMedia
  menubar
  moveBy
  moveTo
  name
  offscreenBuffering
  open
  opener
  orientation
  origin
  outerHeight
  outerWidth
  pageXOffset
  pageYOffset
  parent
  performance
  personalbar
  postMessage
  print
  prompt
  queueMicrotask
  releaseEvents
  requestAnimationFrame
  resizeBy
  resizeTo
  screen
  screenLeft
  screenTop
  screenX
  screenY
  scroll
  scrollbars
  scrollBy
  scrollTo
  scrollX
  scrollY
  sessionStorage
  speechSynthesis
  status
  statusbar
  stop
  styleMedia
  toolbar
  top

  // ignore vendor
  msContentScript
  msWriteProfilerMark
  webkitCancelAnimationFrame
  webkitConvertPointFromNodeToPage
  webkitConvertPointFromPageToNode
  webkitRequestAnimationFrame
}
