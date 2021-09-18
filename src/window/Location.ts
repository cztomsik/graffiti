import { UNSUPPORTED } from '../util'
import { History } from './History'

export class Location implements globalThis.Location {
  #history: History

  constructor(history: History) {
    this.#history = history
  }

  get #url() {
    return new URL(this.#history._current.url)
  }

  get href() {
    return this.#url.href
  }

  set href(v) {
    this.assign(v)
  }

  get pathname() {
    return this.#url.pathname
  }

  set pathname(v) {
    this.assign(v)
  }

  get search() {
    return this.#url.search
  }

  set search(v) {
    this.assign(`?${v}`)
  }

  get hash() {
    return this.#url.hash
  }

  set hash(v) {
    this.assign(`#${v}`)
  }

  get hostname() {
    return this.#url.hostname
  }

  set hostname(hostname) {
    UNSUPPORTED()
  }

  get port() {
    return this.#url.port
  }

  set port(port) {
    UNSUPPORTED()
  }

  get protocol() {
    return this.#url.protocol
  }

  set protocol(protocol) {
    UNSUPPORTED()
  }

  assign(href: string) {
    this.#history._navigate(href, false)
  }

  replace(href: string) {
    this.#history._navigate(href, true)
  }

  reload() {
    // TODO: restart worker
    console.warn('TODO: location.reload()')
  }

  toString() {
    return this.href
  }

  // both vite & wmr need this
  get origin() {
    return this.href.replace(/\/$/, '')
  }

  // TODO: later
  host
  ancestorOrigins
}
