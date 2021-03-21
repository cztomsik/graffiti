import { UNSUPPORTED } from '../util'

export class StyleSheet implements globalThis.StyleSheet {
  constructor(private readonly parent: HTMLStyleElement) {}

  get type() {
    return 'text/css'
  }

  get ownerNode(): HTMLStyleElement {
    // can be null (TS is wrong) after detach or if <style>.textContent has been changed already
    return this.parent.sheet === (this as any) ? this.parent : (null as any)
  }

  get title() {
    return this.ownerNode?.title ?? null
  }

  get disabled() {
    return UNSUPPORTED()
  }

  get parentStyleSheet(): CSSStyleSheet | null {
    console.warn('CSS @import is not supported')

    return null
  }

  get href() {
    // should be original url but we don't support remote CSS
    return UNSUPPORTED()
  }

  get media() {
    // no media queries for now but it could be like:
    //  return new MediaList(this.mediaQueryRules)
    // and it should be abstract & implemented in CSSStyleSheet
    return UNSUPPORTED()
  }
}
