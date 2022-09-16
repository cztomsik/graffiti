export class DOMTokenList implements globalThis.DOMTokenList {
  #element: Element
  #attrName: string;

  // TODO
  [index: number]: string

  constructor(element: Element, attrName: string) {
    this.#element = element
    this.#attrName = attrName
  }

  toString() {
    return this.value
  }

  get value() {
    return this.#element[this.#attrName]
  }

  set value(value) {
    this.#element[this.#attrName] = value
  }

  get #tokens() {
    return this.value.split(/\s+/g)
  }

  set #tokens(tokens: string[]) {
    this.value = tokens.join(' ')
  }

  get length() {
    return this.#tokens.length
  }

  item(index: number) {
    return this.#tokens[index]
  }

  contains(token: string) {
    return this.#tokens.includes(token)
  }

  add(...tokens: string[]) {
    this.#tokens = [...new Set([...this.#tokens, ...tokens])]
  }

  remove(...tokens: string[]) {
    this.#tokens = this.#tokens.filter(t => !tokens.includes(t))
  }

  replace(oldToken: string, newToken: string) {
    let replaced = false
    this.#tokens = this.#tokens.map(t => (t === oldToken ? ((replaced = true), newToken) : t))
    return replaced
  }

  toggle(token: string, force = this.contains(token)) {
    this[force ? 'add' : 'remove'](token)
    return !force
  }

  forEach(callbackFn: (value: string, key: number, parent: DOMTokenList) => void, thisArg?: any) {
    this.#tokens.forEach((v, k) => callbackFn(v, k, this), thisArg)
  }

  supports(token: string) {
    return true
  }
}
