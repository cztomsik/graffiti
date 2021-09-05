import { CSSRule } from './CSSRule'
import { CSSStyleDeclaration } from './CSSStyleDeclaration'
import { native, getNativeId, register } from '../native'

export class CSSStyleRule extends CSSRule implements globalThis.CSSStyleRule {
  #style = register(new CSSStyleDeclaration(this), native.CssStyleRule_style(getNativeId(this)))

  get selectorText() {
    return native.CssStyleRule_selector_text(getNativeId(this))
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
