import { UNSUPPORTED } from '../util'

export class StyleSheet implements globalThis.StyleSheet {
  constructor(public readonly ownerNode: HTMLStyleElement) {}

  get type() {
    return 'text/css'
  }

  get title() {
    return this.ownerNode.title
  }

  get disabled() {
    return UNSUPPORTED()
  }

  get parentStyleSheet() {
    return UNSUPPORTED()
  }

  get href() {
    return UNSUPPORTED()
  }

  get media() {
    return UNSUPPORTED()
  }
}
