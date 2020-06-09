import { StyleSheet } from './StyleSheet'
import { CSSRuleList } from './CSSRuleList'
import { UNSUPPORTED } from '../util'

export class CSSStyleSheet extends StyleSheet implements globalThis.CSSStyleSheet {
  readonly cssRules = new CSSRuleList()

  get ownerRule(): CSSRule | null {
    return UNSUPPORTED()
  }

  get rules() {
    return this.cssRules
  }

  deleteRule(index: number) {
    this.cssRules.splice(index, 1)
  }

  insertRule(rule: string, index: number): number {
    return UNSUPPORTED()
  }

  // deprecated
  cssText
  id
  imports
  isAlternate
  isPrefAlternate
  owningElement
  pages
  readOnly
  addRule
  addImport
  addPageRule
  removeImport
  removeRule
}
