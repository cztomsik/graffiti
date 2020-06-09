import { CSSRule } from './CSSRule'
import { UNSUPPORTED } from '../util'
import { CSSStyleDeclaration } from './CSSStyleDeclaration'

export class CSSStyleRule extends CSSRule implements globalThis.CSSStyleRule {
  readonly style = new CSSStyleDeclaration(this, (prop, value) => console.log('TODO: change rule style', prop, value))

  get type() {
    return CSSRule.STYLE_RULE
  }

  get selectorText() {
    return UNSUPPORTED()
  }

  set selectorText(selectorText: string) {
    UNSUPPORTED()
  }
}
