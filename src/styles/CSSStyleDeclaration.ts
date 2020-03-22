import { SceneContext } from '../core/SceneContext'
import { Node } from '../dom/Node'
import { updateText } from '../dom/Text'
import { UNSUPPORTED } from '../core/utils'
import { SceneChange as S } from '../core/interop'
import { INVALID, parseAlign, parseColor, parseDimension, parseDisplay, parseFlexDirection, parseFlexWrap, parseTransform, parseOverflow } from './parsers'

// TODO: const defaults with defaults for all properties?
// and these should be set if prop is deleted
// and it probably can be used for "resetting" too

// minimal impl just to get something working
//
// for now, only setters are supported
// (and many props are missing)
//
// BTW: in theory parsing CSS values might be faster in native and it's tempting
// to think it could be done async, while the rest of js is still executing
// but in practice, any communication with native involves quite expensive
// context-switch/lock which is why we are batching everything until the end of
// the frame (or when flush is needed) and we want to to do that,
// but it also means it doesn't make sense to parse CSS props/values
// async in native - it's simply not worth and it's definitely much easier to
// do it in js
export class CSSStyleDeclaration implements globalThis.CSSStyleDeclaration {
  // TODO: move to native, each variant for each prop
  _textStyle: any = { fontSize: 16, lineHeight: 20 }

  constructor(private _scene: SceneContext, private _elementId) {}

  getPropertyPriority(prop: string): string {
    return UNSUPPORTED()
  }

  getPropertyValue(prop: string): string {
    return UNSUPPORTED()
  }

  item(index: number): string {
    return UNSUPPORTED()
  }

  get length(): number {
    return UNSUPPORTED()
  }

  // TODO: proxy (or extend array)
  [index: number]: string

  removeProperty(prop: string): string {
    this.setProperty(prop, defaults[prop])
    return ''
  }

  // note we expect only valid names (there's no normalization, error reporting, etc.)
  // but that shouldn't be a problem if you're using modern IDE and/or tooling
  setProperty(prop: string, v?: string | null, priority?: string | null): void {
    if (priority === 'important') {
      console.warn('!important is not supported')
    }

    // break recursion (removeProperty)
    if (v === undefined) {
      return
    }

    // empty -> default (can be different for each property)
    // falsy is common in react ({ backgroundColor: cond && '#fff' })
    if (!v) {
      this.removeProperty(prop)
      return
    }

    // shorthands
    //
    // TODO: introduce native variant for each shorthand
    // (full-form for now)
    switch (prop) {
      case 'flex':
        this.setProperty('flex-grow', v)
        this.setProperty('flex-shrink', v)
        // should be just 0 but chrome does percents too
        this.setProperty('flex-basis', v ? '0%' : 'auto')
        return

      case 'padding':
        this.setProperty('padding-top', v)
        this.setProperty('padding-right', v)
        this.setProperty('padding-bottom', v)
        this.setProperty('padding-left', v)
        return

      case 'margin':
        this.setProperty('margin-top', v)
        this.setProperty('margin-right', v)
        this.setProperty('margin-bottom', v)
        this.setProperty('margin-left', v)
        return

      case 'font-size':
        this._textStyle = { ...this._textStyle, fontSize: parseFloat(v) }
        this._updateTexts()
        return
      case 'line-height':
        this._textStyle = { ...this._textStyle, lineHeight: parseFloat(v) }
        this._updateTexts()
        return
    }

    // not sure how often it happens
    if (typeof v !== 'string') {
      v = '' + v
    }

    const ch = parseChange(prop, v, this._elementId)

    // ignore whole rule if the value cannot be parsed
    if (ch[2] === INVALID) {
      return
    }

    //console.log(prop, v, ch)
    this._scene.changes.push(ch)
  }

  _updateTexts() {
    // update first only (text joining)
    let first = true

    for (const c of (document as any)._getEl(this._elementId).childNodes) {
      if (c.nodeType === Node.TEXT_NODE) {
        if (first) {
          updateText(c)
          first = false
        }
      } else {
        first = true
      }
    }
  }

  set cssText(v) {
    // TODO: mithrill does style.cssText = '' to reset
    if (v !== '') {
      UNSUPPORTED()
    }
  }

  set alignContent(v: string) { this.setProperty('align-content', v) }
  set alignItems(v: string) { this.setProperty('align-items', v) }
  set alignSelf(v: string) { this.setProperty('align-self', v) }
  set background(v: string) { this.setProperty('background', v) }
  set backgroundColor(v: string) { this.setProperty('background-color', v) }
  set border(v: string) { this.setProperty('border', v) }
  set borderBottom(v: string) { this.setProperty('border-bottom', v) }
  set borderBottomColor(v: string) { this.setProperty('border-bottom-color', v) }
  set borderBottomLeftRadius(v: string) { this.setProperty('border-bottom-left-radius', v) }
  set borderBottomRightRadius(v: string) { this.setProperty('border-bottom-right-radius', v) }
  set borderBottomStyle(v: string) { this.setProperty('border-bottom-style', v) }
  set borderBottomWidth(v: string) { this.setProperty('border-bottom-width', v) }
  set borderColor(v: string) { this.setProperty('border-color', v) }
  set borderLeft(v: string) { this.setProperty('border-left', v) }
  set borderLeftColor(v: string) { this.setProperty('border-left-color', v) }
  set borderLeftStyle(v: string) { this.setProperty('border-left-style', v) }
  set borderLeftWidth(v: string) { this.setProperty('border-left-width', v) }
  set borderRadius(v: string) { this.setProperty('border-radius', v) }
  set borderRight(v: string) { this.setProperty('border-right', v) }
  set borderRightColor(v: string) { this.setProperty('border-right-color', v) }
  set borderRightStyle(v: string) { this.setProperty('border-right-style', v) }
  set borderRightWidth(v: string) { this.setProperty('border-right-width', v) }
  set borderStyle(v: string) { this.setProperty('border-style', v) }
  set borderTop(v: string) { this.setProperty('border-top', v) }
  set borderTopColor(v: string) { this.setProperty('border-top-color', v) }
  set borderTopLeftRadius(v: string) { this.setProperty('border-top-left-radius', v) }
  set borderTopRightRadius(v: string) { this.setProperty('border-top-right-radius', v) }
  set borderTopStyle(v: string) { this.setProperty('border-top-style', v) }
  set borderTopWidth(v: string) { this.setProperty('border-top-width', v) }
  set borderWidth(v: string) { this.setProperty('border-width', v) }
  set bottom(v: string) { this.setProperty('bottom', v) }
  set boxShadow(v: string) { this.setProperty('box-shadow', v) }
  set color(v: string) { this.setProperty('color', v) }
  set display(v: string) { this.setProperty('display', v) }
  set flex(v: string) { this.setProperty('flex', v) }
  set flexBasis(v: string) { this.setProperty('flex-basis', v) }
  set flexDirection(v: string) { this.setProperty('flex-direction', v) }
  set flexFlow(v: string) { this.setProperty('flex-flow', v) }
  set flexGrow(v: string) { this.setProperty('flex-grow', v) }
  set flexShrink(v: string) { this.setProperty('flex-shrink', v) }
  set flexWrap(v: string) { this.setProperty('flex-wrap', v) }
  set font(v: string) { this.setProperty('font', v) }
  set fontFamily(v: string) { this.setProperty('font-family', v) }
  set fontSize(v: string) { this.setProperty('font-size', v) }
  set fontStyle(v: string) { this.setProperty('font-style', v) }
  set fontVariant(v: string) { this.setProperty('font-variant', v) }
  set fontWeight(v: string) { this.setProperty('font-weight', v) }
  set height(v: string) { this.setProperty('height', v) }
  set justifyContent(v: string) { this.setProperty('justify-content', v) }
  set left(v: string) { this.setProperty('left', v) }
  set lineHeight(v: string) { this.setProperty('line-height', v) }
  set margin(v: string) { this.setProperty('margin', v) }
  set marginBottom(v: string) { this.setProperty('margin-bottom', v) }
  set marginLeft(v: string) { this.setProperty('margin-left', v) }
  set marginRight(v: string) { this.setProperty('margin-right', v) }
  set marginTop(v: string) { this.setProperty('margin-top', v) }
  set maxHeight(v: string) { this.setProperty('max-height', v) }
  set maxWidth(v: string) { this.setProperty('max-width', v) }
  set minHeight(v: string) { this.setProperty('min-height', v) }
  set minWidth(v: string) { this.setProperty('min-width', v) }
  set overflow(v: string) { this.setProperty('overflow', v) }
  set padding(v: string) { this.setProperty('padding', v) }
  set paddingBottom(v: string) { this.setProperty('padding-bottom', v) }
  set paddingLeft(v: string) { this.setProperty('padding-left', v) }
  set paddingRight(v: string) { this.setProperty('padding-right', v) }
  set paddingTop(v: string) { this.setProperty('padding-top', v) }
  set right(v: string) { this.setProperty('right', v) }
  set textAlign(v: string) { this.setProperty('text-align', v) }
  set top(v: string) { this.setProperty('top', v) }
  set transform(v: string) { this.setProperty('transform', v) }
  set width(v: string) { this.setProperty('width', v) }

  readonly parentRule: CSSRule

  // maybe later (lot of them are SVG-only)
  backgroundAttachment
  backgroundClip
  backgroundPosition
  backgroundPositionX
  backgroundPositionY
  backgroundRepeat
  backgroundSize
  letterSpacing
  opacity
  outline
  outlineColor
  outlineOffset
  outlineStyle
  outlineWidth
  overflowAnchor
  overflowWrap
  overflowX
  overflowY
  pointerEvents
  position
  textTransform
  textUnderlinePosition
  transformBox
  transformOrigin
  transformStyle
  visibility
  whiteSpace
  alignmentBaseline
  animation
  animationDelay
  animationDirection
  animationDuration
  animationFillMode
  animationIterationCount
  animationName
  animationPlayState
  animationTimingFunction
  backfaceVisibility
  baselineShift
  backgroundImage
  backgroundOrigin
  blockSize
  borderBlockEnd
  borderBlockEndColor
  borderBlockEndStyle
  borderBlockEndWidth
  borderBlockStart
  borderBlockStartColor
  borderBlockStartStyle
  borderBlockStartWidth
  borderCollapse
  borderImage
  borderImageOutset
  borderImageRepeat
  borderImageSlice
  borderImageSource
  borderImageWidth
  borderInlineEnd
  borderInlineEndColor
  borderInlineEndStyle
  borderInlineEndWidth
  borderInlineStart
  borderInlineStartColor
  borderInlineStartStyle
  borderInlineStartWidth
  borderSpacing
  boxSizing
  breakAfter
  breakBefore
  breakInside
  captionSide
  caretColor
  clear
  clip
  clipPath
  clipRule
  colorInterpolation
  colorInterpolationFilters
  columnCount
  columnFill
  columnGap
  columnRule
  columnRuleColor
  columnRuleStyle
  columnRuleWidth
  columns
  columnSpan
  columnWidth
  content
  counterIncrement
  counterReset
  cssFloat
  cssText
  cursor
  direction
  dominantBaseline
  emptyCells
  enableBackground
  fill
  fillOpacity
  fillRule
  filter
  float
  floodColor
  floodOpacity
  fontFeatureSettings
  fontKerning
  fontSizeAdjust
  fontStretch
  fontSynthesis
  fontVariantCaps
  fontVariantEastAsian
  fontVariantLigatures
  fontVariantNumeric
  fontVariantPosition
  gap
  glyphOrientationHorizontal
  glyphOrientationVertical
  grid
  gridArea
  gridAutoColumns
  gridAutoFlow
  gridAutoRows
  gridColumn
  gridColumnEnd
  gridColumnGap
  gridColumnStart
  gridGap
  gridRow
  gridRowEnd
  gridRowGap
  gridRowStart
  gridTemplate
  gridTemplateAreas
  gridTemplateColumns
  gridTemplateRows
  hyphens
  imageOrientation
  imageRendering
  imeMode
  inlineSize
  justifySelf
  justifyItems
  kerning
  layoutGrid
  layoutGridChar
  layoutGridLine
  layoutGridMode
  layoutGridType
  lightingColor
  lineBreak
  listStyle
  listStyleImage
  listStylePosition
  listStyleType
  marginBlockEnd
  marginBlockStart
  marginInlineEnd
  marginInlineStart
  marker
  markerEnd
  markerMid
  markerStart
  mask
  maskComposite
  maskImage
  maskPosition
  maskRepeat
  maskSize
  maskType
  maxBlockSize
  maxInlineSize
  minBlockSize
  minInlineSize
  objectFit
  objectPosition
  order
  orphans
  paddingBlockEnd
  paddingBlockStart
  paddingInlineEnd
  paddingInlineStart
  pageBreakAfter
  pageBreakBefore
  pageBreakInside
  paintOrder
  penAction
  perspective
  perspectiveOrigin
  placeContent
  placeItems
  placeSelf
  quotes
  resize
  rotate
  rowGap
  rubyAlign
  rubyOverhang
  rubyPosition
  scale
  scrollBehavior
  shapeRendering
  stopColor
  stopOpacity
  stroke
  strokeDasharray
  strokeDashoffset
  strokeLinecap
  strokeLinejoin
  strokeMiterlimit
  strokeOpacity
  strokeWidth
  tableLayout
  tabSize
  textAlignLast
  textAnchor
  textCombineUpright
  textDecoration
  textDecorationColor
  textDecorationLine
  textDecorationStyle
  textEmphasis
  textEmphasisColor
  textEmphasisPosition
  textEmphasisStyle
  textIndent
  textJustify
  textKashida
  textKashidaSpace
  textOrientation
  textOverflow
  textRendering
  textShadow
  touchAction
  transition
  transitionDelay
  transitionDuration
  transitionProperty
  transitionTimingFunction
  translate
  unicodeBidi
  userSelect
  verticalAlign
  widows
  willChange
  wordBreak
  wordSpacing
  wordWrap
  writingMode
  zIndex
  zoom

  // ignore vendor extensions
  msContentZoomChaining
  msContentZooming
  msContentZoomLimit
  msContentZoomLimitMax
  msContentZoomLimitMin
  msContentZoomSnap
  msContentZoomSnapPoints
  msContentZoomSnapType
  msFlowFrom
  msFlowInto
  msFontFeatureSettings
  msGridColumn
  msGridColumnAlign
  msGridColumns
  msGridColumnSpan
  msGridRow
  msGridRowAlign
  msGridRows
  msGridRowSpan
  msHighContrastAdjust
  msHyphenateLimitChars
  msHyphenateLimitLines
  msHyphenateLimitZone
  msHyphens
  msImeAlign
  msOverflowStyle
  msScrollChaining
  msScrollLimit
  msScrollLimitXMax
  msScrollLimitXMin
  msScrollLimitYMax
  msScrollLimitYMin
  msScrollRails
  msScrollSnapPointsX
  msScrollSnapPointsY
  msScrollSnapType
  msScrollSnapX
  msScrollSnapY
  msScrollTranslation
  msTextCombineHorizontal
  msTextSizeAdjust
  msTouchAction
  msTouchSelect
  msUserSelect
  msWrapFlow
  msWrapMargin
  msWrapThrough
  webkitAlignContent
  webkitAlignItems
  webkitAlignSelf
  webkitAnimation
  webkitAnimationDelay
  webkitAnimationDirection
  webkitAnimationDuration
  webkitAnimationFillMode
  webkitAnimationIterationCount
  webkitAnimationName
  webkitAnimationPlayState
  webkitAnimationTimingFunction
  webkitAppearance
  webkitBackfaceVisibility
  webkitBackgroundClip
  webkitBackgroundOrigin
  webkitBackgroundSize
  webkitBorderBottomLeftRadius
  webkitBorderBottomRightRadius
  webkitBorderImage
  webkitBorderRadius
  webkitBorderTopLeftRadius
  webkitBorderTopRightRadius
  webkitBoxAlign
  webkitBoxDirection
  webkitBoxFlex
  webkitBoxOrdinalGroup
  webkitBoxOrient
  webkitBoxPack
  webkitBoxShadow
  webkitBoxSizing
  webkitColumnBreakAfter
  webkitColumnBreakBefore
  webkitColumnBreakInside
  webkitColumnCount
  webkitColumnGap
  webkitColumnRule
  webkitColumnRuleColor
  webkitColumnRuleStyle
  webkitColumnRuleWidth
  webkitColumns
  webkitColumnSpan
  webkitColumnWidth
  webkitFilter
  webkitFlex
  webkitFlexBasis
  webkitFlexDirection
  webkitFlexFlow
  webkitFlexGrow
  webkitFlexShrink
  webkitFlexWrap
  webkitJustifyContent
  webkitLineClamp
  webkitMask
  webkitMaskBoxImage
  webkitMaskBoxImageOutset
  webkitMaskBoxImageRepeat
  webkitMaskBoxImageSlice
  webkitMaskBoxImageSource
  webkitMaskBoxImageWidth
  webkitMaskClip
  webkitMaskComposite
  webkitMaskImage
  webkitMaskOrigin
  webkitMaskPosition
  webkitMaskRepeat
  webkitMaskSize
  webkitOrder
  webkitPerspective
  webkitPerspectiveOrigin
  webkitTapHighlightColor
  webkitTextFillColor
  webkitTextSizeAdjust
  webkitTextStroke
  webkitTextStrokeColor
  webkitTextStrokeWidth
  webkitTransform
  webkitTransformOrigin
  webkitTransformStyle
  webkitTransition
  webkitTransitionDelay
  webkitTransitionDuration
  webkitTransitionProperty
  webkitTransitionTimingFunction
  webkitUserModify
  webkitUserSelect
  webkitWritingMode
}

const defaults = {
  // TODO
  'background-color': '#0000'
}

// indirection so it's monomorphic
function parseChange(prop: string, v: string, _id) {
    // TODO: order by likelihood

    // TODO: parse shorthands -> ShorthandVariant

    switch (prop) {
      case 'align-content':
        return S.AlignContent(_id, parseAlign(v))
      case 'align-items':
        return S.AlignItems(_id, parseAlign(v))
      case 'align-self':
        return S.AlignSelf(_id, parseAlign(v))
      //case 'background':
      //  return S.Background(_id, parse(v))
      case 'background-color':
        return S.BackgroundColor(_id, parseColor(v))
      //case 'border':
      //  return S.Border(_id, parse(v))
      //case 'border-bottom':
      //  return S.BorderBottom(_id, parse(v))
      //case 'border-bottom-color':
      //  return S.BorderBottomColor(_id, parseColor(v))
      //case 'border-bottom-left-radius':
      //  return S.BorderBottomLeftRadius(_id, parse(v))
      //case 'border-bottom-right-radius':
      //  return S.BorderBottomRightRadius(_id, parse(v))
      //case 'border-bottom-style':
      //  return S.BorderBottomStyle(_id, parse(v))
      //case 'border-bottom-width':
      //  return S.BorderBottomWidth(_id, parse(v))
      //case 'border-color':
      //  return S.BorderColor(_id, parseColor(v))
      //case 'border-left':
      //  return S.BorderLeft(_id, parse(v))
      //case 'border-left-color':
      //  return S.BorderLeftColor(_id, parseColor(v))
      //case 'border-left-style':
      //  return S.BorderLeftStyle(_id, parse(v))
      //case 'border-left-width':
      //  return S.BorderLeftWidth(_id, parse(v))
      //case 'border-radius':
      //  return S.BorderRadius(_id, parse(v))
      //case 'border-right':
      //  return S.BorderRight(_id, parse(v))
      //case 'border-right-color':
      //  return S.BorderRightColor(_id, parseColor(v))
      //case 'border-right-style':
      //  return S.BorderRightStyle(_id, parse(v))
      //case 'border-right-width':
      //  return S.BorderRightWidth(_id, parse(v))
      //case 'border-style':
      //  return S.BorderStyle(_id, parse(v))
      //case 'border-top':
      //  return S.BorderTop(_id, parse(v))
      //case 'border-top-color':
      //  return S.BorderTopColor(_id, parseColor(v))
      //case 'border-top-left-radius':
      //  return S.BorderTopLeftRadius(_id, parse(v))
      //case 'border-top-right-radius':
      //  return S.BorderTopRightRadius(_id, parse(v))
      //case 'border-top-style':
      //  return S.BorderTopStyle(_id, parse(v))
      //case 'border-top-width':
      //  return S.BorderTopWidth(_id, parse(v))
      //case 'border-width':
      //  return S.BorderWidth(_id, parse(v))
      case 'bottom':
        return S.Bottom(_id, parseDimension(v))
      //case 'box-shadow':
      //  return S.BoxShadow(_id, parse(v))
      case 'color':
        return S.Color(_id, parseColor(v))
      case 'display':
        return S.Display(_id, parseDisplay(v))
      //case 'flex':
      //  return S.Flex(_id, parseFlex(v))
      case 'flex-basis':
        return S.FlexBasis(_id, parseDimension(v))
      case 'flex-direction':
        return S.FlexDirection(_id, parseFlexDirection(v))
      //case 'flex-flow':
      //  return S.FlexFlow(_id, parse(v))
      case 'flex-grow':
        return S.FlexGrow(_id, +v)
      case 'flex-shrink':
        return S.FlexShrink(_id, +v)
      case 'flex-wrap':
        return S.FlexWrap(_id, parseFlexWrap(v))
      //case 'font':
      //  return S.Font(_id, parse(v))
      //case 'font-family':
      //  return S.FontFamily(_id, parse(v))
      //case 'font-size':
      //  return S.FontSize(_id, parse(v))
      //case 'font-style':
      //  return S.FontStyle(_id, parse(v))
      //case 'font-variant':
      //  return S.FontVariant(_id, parse(v))
      //case 'font-weight':
      //  return S.FontWeight(_id, parse(v))
      case 'height':
        return S.Height(_id, parseDimension(v))
      case 'justify-content':
        return S.JustifyContent(_id, parseAlign(v))
      case 'left':
        return S.Left(_id, parseDimension(v))
      //case 'line-height':
      //  return S.LineHeight(_id, parse(v))
      //case 'margin':
      //  return S.Margin(_id, parse(v))
      case 'margin-bottom':
        return S.MarginBottom(_id, parseDimension(v))
      case 'margin-left':
        return S.MarginLeft(_id, parseDimension(v))
      case 'margin-right':
        return S.MarginRight(_id, parseDimension(v))
      case 'margin-top':
        return S.MarginTop(_id, parseDimension(v))
      case 'max-height':
        return S.MaxHeight(_id, parseDimension(v))
      case 'max-width':
        return S.MaxWidth(_id, parseDimension(v))
      case 'min-height':
        return S.MinHeight(_id, parseDimension(v))
      case 'min-width':
        return S.MinWidth(_id, parseDimension(v))
      case 'overflow':
        return S.Overflow(_id, parseOverflow(v))
      //case 'padding':
      //  return S.Padding(_id, parse(v))
      case 'padding-bottom':
        return S.PaddingBottom(_id, parseDimension(v))
      case 'padding-left':
        return S.PaddingLeft(_id, parseDimension(v))
      case 'padding-right':
        return S.PaddingRight(_id, parseDimension(v))
      case 'padding-top':
        return S.PaddingTop(_id, parseDimension(v))
      case 'right':
        return S.Right(_id, parseDimension(v))
      //case 'text-align':
      //  return S.TextAlign(_id, parseAlign(v))
      case 'top':
        return S.Top(_id, parseDimension(v))
      case 'transform':
        return S.Transform(_id, parseTransform(v))
      case 'width':
        return S.Width(_id, parseDimension(v))
    }

    return [,,INVALID]
}
