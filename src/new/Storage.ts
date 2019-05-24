export class Storage implements globalThis.Storage {
  constructor() {}

  key(n) {
    return Object.keys(this)[n]
  }

  get length() {
    return Object.keys(this).length
  }

  getItem(key) {
    return this[key]
  }

  setItem(key, value) {
    this[key] = value
  }

  removeItem(key) {
    delete this[key]
  }

  clear() {
    for (const k in this) {
      delete this[k]
    }
  }
}
