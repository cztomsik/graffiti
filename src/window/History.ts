import { Event } from '../events/Event'

export class History implements globalThis.History {
  #window
  #states: any[] = []
  #index = 0

  constructor(window) {
    this.#window = window

    // initial state
    this.#states.push({
      data: undefined,
      title: '',
      get url() {
        return window.document.URL
      }
    })

    // TODO: set title on load?
  }

  back() {
    this.go(-1)
  }

  forward() {
    this.go(1)
  }

  get length() {
    // TODO: not sure about this
    return Math.min(this.#index, this.#states.length)
  }

  get state() {
    return this._current.data
  }

  get _current() {
    return this.#states[this.#index]
  }

  go(delta?) {
    // should be async but return void
    setTimeout(() => {
      if (!delta) {
        return location.reload()
      }

      this._notifyAfterTransition({
        from: this._current,
        to: this.#states[this.#index = Math.max(0, this.#index + delta)]
      })
    })
  }

  pushState(data, title, url) {
    const state = this._resolve(data, title, url)
    // strip & advance
    this.#states[this.#states.length = ++this.#index] = state
  }

  replaceState(data, title, url) {
    // TODO: not sure if it should keep the stack
    this.#states[this.#index] = this._resolve(data, title, url)
  }

  _navigate(href, replace) {
    // save because it could be replaced
    const from = this._current

    if (replace) {
      this.replaceState(undefined, null, href)
    } else {
      this.pushState(undefined, null, href)
    }

    this._notifyAfterTransition({ from, to: this._current })
  }

  _notifyAfterTransition({ from, to }) {
    // console.log(`transition from ${from.url.href} to ${to.url.href}`)

    if (from === to) {
      return
    }

    this.#window.dispatchEvent(
      Object.assign(new Event('popstate'), { state: to.data })
    )

    if ((from.url.href === to.url.href) && (from.url.hash !== to.url.hash)) {
      this.#window.dispatchEvent(
        Object.assign(new Event('hashchange'), {
          oldURL: from.url.href,
          newURL: to.url.href
        })
      )
    }

    if (to.title) {
      this.#window.document.title = to.title
    }
  }

  _resolve(data = undefined, title = '', url: any = '') {
    url = new URL(url, this._current.url)

    return { data, title, url }
  }

  // TODO: later
  scrollRestoration
}
