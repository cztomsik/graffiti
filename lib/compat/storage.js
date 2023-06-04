// super-simple, in-memory polyfill for localStorage and sessionStorage

class Storage {
  get length() {
    return Object.keys(this).length
  }

  clear() {
    Object.keys(this).forEach(k => this.removeItem(k))
  }

  getItem(key) {
    return this[key] ? '' + this[key] : null
  }

  key(index) {
    return Object.keys(this)[index] ?? null
  }

  removeItem(key) {
    delete this[key]
  }

  setItem(key, value = '') {
    this[key] = '' + value
  }
}

globalThis.Storage = Storage
globalThis.localStorage = new Storage()
globalThis.sessionStorage = new Storage()
