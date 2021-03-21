import { CSSRule } from './CSSRule'
import { CSSStyleDeclaration } from './CSSStyleDeclaration'

export class CSSStyleRule extends CSSRule implements globalThis.CSSStyleRule {
  readonly style = new CSSStyleDeclaration(this, (prop, value) => console.log('TODO: change rule style', prop, value))

  constructor(parent: CSSStyleSheet, public selectorText: string) {
    super(parent)
  }

  get type() {
    return CSSRule.STYLE_RULE
  }

  get cssText() {
    return `${this.selectorText} { ${this.style.cssText} }`
  }
}
