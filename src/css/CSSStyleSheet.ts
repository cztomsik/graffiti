import { StyleSheet } from './StyleSheet'
import { CSSRuleList } from './CSSRuleList'
import { CSSStyleRule } from './CSSStyleRule'

export class CSSStyleSheet extends StyleSheet implements globalThis.CSSStyleSheet {
  readonly cssRules = new CSSRuleList()

  constructor(parent: HTMLStyleElement, private _onChange) {
    super(parent)
  }

  get ownerRule(): CSSRule | null {
    console.warn('CSS @import is not supported')

    return null
  }

  get rules() {
    return this.cssRules
  }

  insertRule(rule: string, index = 0): number {
    //this.insertRules(rule, index)

    return index
  }

  /*
  insertRules(str, index = 0) {
    const rules = parseRules(str).map(({ selector, props }) => {
      const rule = new CSSStyleRule(this, selector)

      Object.entries(props).forEach(([k, v]) => rule.style.setProperty(k, v))

      return rule
    })

    this.cssRules.splice(index, 0, ...rules)
  }
  */

  deleteRule(index: number) {
    this.cssRules.splice(index, 1)
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
