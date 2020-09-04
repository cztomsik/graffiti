import { CSSRule } from './CSSRule'
import { CSSStyleDeclaration } from './CSSStyleDeclaration'

export class CSSStyleRule extends CSSRule implements globalThis.CSSStyleRule {
  _selectorText: string
  readonly style = new CSSStyleDeclaration(this, (prop, value) => console.log('TODO: change rule style', prop, value))

  constructor(parent: CSSStyleSheet, selectorText: string) {
    super(parent)

    this._selectorText = selectorText
  }

  get type() {
    return CSSRule.STYLE_RULE
  }

  get selectorText() {
    return this._selectorText
  }

  set selectorText(selectorText: string) {
    this._selectorText = selectorText

    console.log('TODO: notify selectorText changed')
  }

  get cssText() {
    return `${this.selectorText} { ${this.style.cssText} }`
  }
}
