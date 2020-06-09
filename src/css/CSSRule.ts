import { UNSUPPORTED } from '../util'

export abstract class CSSRule implements globalThis.CSSRule {
  abstract readonly type: number

  // TODO
  get cssText() {
    return UNSUPPORTED()
  }

  // no-op https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/cssText
  set cssText(v: string) {}

  get parentRule() {
    return UNSUPPORTED()
  }

  get parentStyleSheet() {
    return UNSUPPORTED()
  }

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

  readonly UNKNOWN_RULE = CSSRule.UNKNOWN_RULE
  readonly STYLE_RULE = CSSRule.STYLE_RULE
  readonly CHARSET_RULE = CSSRule.CHARSET_RULE
  readonly IMPORT_RULE = CSSRule.IMPORT_RULE
  readonly MEDIA_RULE = CSSRule.MEDIA_RULE
  readonly FONT_FACE_RULE = CSSRule.FONT_FACE_RULE
  readonly PAGE_RULE = CSSRule.PAGE_RULE
  readonly KEYFRAMES_RULE = CSSRule.KEYFRAMES_RULE
  readonly KEYFRAME_RULE = CSSRule.KEYFRAME_RULE
  readonly NAMESPACE_RULE = CSSRule.NAMESPACE_RULE
  readonly SUPPORTS_RULE = CSSRule.SUPPORTS_RULE
  readonly VIEWPORT_RULE = CSSRule.VIEWPORT_RULE
}
