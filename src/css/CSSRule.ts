export abstract class CSSRule implements globalThis.CSSRule {
  abstract readonly type: number
  abstract readonly cssText: string

  constructor(private readonly parent: CSSStyleSheet) {}

  get parentStyleSheet(): CSSStyleSheet | null {
    // null if it has been removed already
    return Array.from(this.parent.cssRules).includes(this) ?this.parent :null
  }

  get parentRule(): CSSRule | null {
    console.warn('CSS @import is not supported')

    return null
  }

  // rule types
  static readonly UNKNOWN_RULE = 0
  static readonly STYLE_RULE = 1
  static readonly CHARSET_RULE = 2
  static readonly IMPORT_RULE = 3
  static readonly MEDIA_RULE = 4
  static readonly FONT_FACE_RULE = 5
  static readonly PAGE_RULE = 6
  static readonly KEYFRAMES_RULE = 7
  static readonly KEYFRAME_RULE = 8
  static readonly NAMESPACE_RULE = 10
  static readonly SUPPORTS_RULE = 12
  static readonly VIEWPORT_RULE = 15

  // types again (instance)
  // (getters are defined on prototype so they don't consume instance space)
  get UNKNOWN_RULE() { return CSSRule.UNKNOWN_RULE }
  get STYLE_RULE() { return CSSRule.STYLE_RULE }
  get CHARSET_RULE() { return CSSRule.CHARSET_RULE }
  get IMPORT_RULE() { return CSSRule.IMPORT_RULE }
  get MEDIA_RULE() { return CSSRule.MEDIA_RULE }
  get FONT_FACE_RULE() { return CSSRule.FONT_FACE_RULE }
  get PAGE_RULE() { return CSSRule.PAGE_RULE }
  get KEYFRAMES_RULE() { return CSSRule.KEYFRAMES_RULE }
  get KEYFRAME_RULE() { return CSSRule.KEYFRAME_RULE }
  get NAMESPACE_RULE() { return CSSRule.NAMESPACE_RULE }
  get SUPPORTS_RULE() { return CSSRule.SUPPORTS_RULE }
  get VIEWPORT_RULE() { return CSSRule.VIEWPORT_RULE }
}
