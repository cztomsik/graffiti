import { CSSRule } from './CSSRule'
import { CSSStyleDeclaration } from './CSSStyleDeclaration'
import { TODO } from '../util'

export class CSSStyleRule extends CSSRule implements globalThis.CSSStyleRule {
  #style = new CSSStyleDeclaration(this)

  get selectorText() {
    return TODO()
  }

  get style() {
    return this.#style
  }

  get type() {
    return CSSRule.STYLE_RULE
  }

  get cssText() {
    return `${this.selectorText} { ${this.style.cssText} }`
  }
}
