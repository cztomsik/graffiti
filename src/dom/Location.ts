import { URL } from 'url'
import { Event } from '../events/Event'

export class Location {
  _url = new URL('noprotocol://nohost/')

  constructor(private window) {}

  get href() {
    return this._url.href
  }

  set href(v) {
    //console.log('href =', v)
    const oldURL = this.href

    this._url = new URL(v, this.href)

    this.window.dispatchEvent(
      Object.assign(new Event('hashchange'), {
        oldURL,
        newURL: this.href
      })
    )
  }

  get pathname() {
    return this._url.pathname
  }

  set pathname(v) {
    this.href = v
  }

  get search() {
    return this._url.search
  }

  set search(v) {
    this.href = `?${v}`
  }

  get hash() {
    return this._url.hash
  }

  set hash(v) {
    this.href = `#${v}`
  }

  assign(href) {
    this.href = href
  }

  reload() {
    this.href = this.href
  }

  replace(href) {
    this.href = href
  }

  toString() {
    return this.href
  }
}
