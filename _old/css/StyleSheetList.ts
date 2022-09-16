export class StyleSheetList extends Array<CSSStyleSheet> implements globalThis.StyleSheetList {
  item(index: number): CSSStyleSheet | null {
    return this[index] ?? null
  }
}
