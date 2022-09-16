export class Storage implements globalThis.Storage {
  [name: string]: any

  get length() {
    return Object.keys(this).length
  }

  clear() {
    Object.keys(this).forEach(k => this.removeItem(k))
  }

  getItem(key: string): string | null {
    return this[key] ? '' + this[key] : null
  }

  key(index: number): string | null {
    return Object.keys(this)[index] ?? null
  }

  removeItem(key: string) {
    delete this[key]
  }

  setItem(key: string, value = '') {
    this[key] = '' + value
  }
}
