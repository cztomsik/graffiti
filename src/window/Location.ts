import { UNSUPPORTED } from '../util'
import { History } from './History'

export class Location implements globalThis.Location {
  #history: History

  constructor(history: History) {
    this.#history = history
  }

  get _url() {
    return this.#history._current.url
  }

  get href() {
    return this._url.href
  }

  set href(v) {
    this.assign(v)
  }

  get pathname() {
    return this._url.pathname
  }

  set pathname(v) {
    this.assign(v)
  }

  get search() {
    return this._url.search
  }

  set search(v) {
    this.assign(`?${v}`)
  }

  get hash() {
    return this._url.hash
  }

  set hash(v) {
    this.assign(`#${v}`)
  }

  get hostname() {
    return this._url.hostname
  }

  set hostname(hostname) {
    UNSUPPORTED()
  }

  get port() {
    return this._url.port
  }

  set port(port) {
    UNSUPPORTED()
  }

  get protocol() {
    return this._url.protocol
  }

  set protocol(protocol) {
    UNSUPPORTED()
  }

  assign(href) {
    this.#history._navigate(href, false)
  }

  replace(href) {
    this.#history._navigate(href, true)
  }

  reload() {
    // TODO: restart worker
    console.warn('TODO: location.reload()')
  }

  toString() {
    return this.href
  }

  // TODO: later
  host
  origin
  ancestorOrigins
}
