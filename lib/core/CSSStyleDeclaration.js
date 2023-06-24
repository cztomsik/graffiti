// TODO: https://github.com/cztomsik/graffiti/blob/bd1dfe61d3d7b5bfbf9184ecfb9e068dda982a60/src/css/CSSStyleDeclaration.ts

import { native } from './native.js'

export class CSSStyleDeclaration {
  ownerNode = null

  get length() {
    return native.CSSStyleDeclaration_length(this)
  }

  item(i) {
    return native.CSSStyleDeclaration_item(this, i)
  }

  getPropertyValue(prop) {
    return native.CSSStyleDeclaration_getPropertyValue(this, prop)
  }

  setProperty(prop, value) {
    native.CSSStyleDeclaration_setProperty(this, prop, '' + value)
    native.Node_markDirty(this.ownerNode)
  }

  removeProperty(prop) {
    native.CSSStyleDeclaration_removeProperty(this, prop)
    native.Node_markDirty(this.ownerNode)
  }

  get cssText() {
    return native.CSSStyleDeclaration_cssText(this)
  }

  set cssText(cssText) {
    native.CSSStyleDeclaration_setCssText(this, cssText)
  }
}

// TODO(perf): try to get list of properties from native and define getters/setters
Object.setPrototypeOf(
  CSSStyleDeclaration.prototype,
  new Proxy(Object.getPrototypeOf(CSSStyleDeclaration.prototype), {
    get: (_, k, style) => (style.getPropertyValue(kebabCase(String(k))), true),
    set: (_, k, v, style) => (style.setProperty(kebabCase(String(k)), v), true),
  })
)

const kebabCase = s => s.replace(/[A-Z]/g, c => '-' + c.toLowerCase())
