export class CSSRuleList extends Array<CSSRule> implements globalThis.CSSRuleList {
  item(index: number): CSSRule | null {
    return this[index] ?? null
  }
}
