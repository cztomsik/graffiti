import { UNSUPPORTED } from '../util'

export class StyleSheet implements globalThis.StyleSheet {
  #parent

  constructor(parent: HTMLStyleElement) {
    this.#parent = parent
  }

  get type() {
    return 'text/css'
  }

  // should be null after detach or if <style>.textContent has been changed already
  // but we don't support that
  get ownerNode(): HTMLStyleElement {
    return this.#parent
  }

  get title() {
    return this.ownerNode?.title ?? null
  }

  get disabled() {
    return false
  }

  get parentStyleSheet(): CSSStyleSheet | null {
    console.warn('CSS @import is not supported')

    return null
  }

  get href() {
    return null
  }

  get media() {
    // no media queries for now but it could be like:
    //  return new MediaList(this.mediaQueryRules)
    // and it should be abstract & implemented in CSSStyleSheet
    return UNSUPPORTED()
  }
}
