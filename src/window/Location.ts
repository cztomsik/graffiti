import { History } from './History'

export class Location implements globalThis.Location {
  constructor(private _history: History) {}

  get _url() {
    return this._history._current.url
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

  assign(href) {
    this._history._navigate(href, false)
  }

  replace(href) {
    this._history._navigate(href, true)
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
  hostname
  port
  protocol
  origin
  ancestorOrigins
}
