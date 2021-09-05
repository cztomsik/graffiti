import { StyleSheet } from './StyleSheet'
import { CSSRuleList } from './CSSRuleList'
import { CSSStyleRule } from './CSSStyleRule'

export class CSSStyleSheet extends StyleSheet implements globalThis.CSSStyleSheet {
  readonly cssRules = new CSSRuleList()

  get ownerRule(): CSSRule | null {
    console.warn('CSS @import is not supported')

    return null
  }

  insertRule(rule: string, index = 0): number {
    native.CssStyleSheet_insert_rule(getNativeId(this), rule, index)

    return index
  }

  deleteRule(index: number) {
    native.CssStyleSheet_delete_rule(getNativeId(this), index)
  }

  // deprecated
  rules

  // deprecated
  addRule(sel, style, index = this.cssRules.length) {
    this.insertRule(`${sel} { $style }`, index)
    return -1
  }

  // deprecated
  removeRule(index) {
    this.deleteRule(index)
  }
}
