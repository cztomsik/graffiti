import { StyleSheet } from './StyleSheet'
import { CSSRuleList } from './CSSRuleList'
import { CSSStyleRule } from './CSSStyleRule'
import { native, encode, getNativeId } from '../native'
import { TODO } from '../util'

export class CSSStyleSheet extends StyleSheet implements globalThis.CSSStyleSheet {
  readonly cssRules = new CSSRuleList()

  get ownerRule(): CSSRule | null {
    console.warn('CSS @import is not supported')

    return null
  }

  insertRule(rule: string, index = 0): number {
    native.gft_CssStyleSheet_insert_rule(getNativeId(this.ownerNode), ...encode(rule), index)

    return index
  }

  deleteRule(index: number) {
    native.gft_CssStyleSheet_delete_rule(getNativeId(this.ownerNode), index)
  }

  // deprecated
  rules

  // deprecated
  addRule(sel, style, index = this.cssRules.length) {
    this.insertRule(`${sel} { ${style} }`, index)
    return -1
  }

  // deprecated
  removeRule(index) {
    this.deleteRule(index)
  }
}
