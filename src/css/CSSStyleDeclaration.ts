import { UNSUPPORTED } from '../util'
import { parseRules } from './parse'

// minimal impl just to get something working
// (many props are missing)
export class CSSStyleDeclaration implements globalThis.CSSStyleDeclaration {
  _values = new Map<string, string>()
  _onChange

  constructor(public readonly parentRule, onChange) {
    this._onChange = onChange
  }

  getPropertyValue(propertyName: string): string {
    return this._values.get(propertyName) ?? ''
  }

  getPropertyPriority(propertyName: string): string {
    return UNSUPPORTED()
  }

  setProperty(propertyName: string, value: string | null, priority?: string | null) {
    if (value === '' || value === null) {
      this.removeProperty(propertyName)
      return
    }

    if (priority === 'important') {
      console.warn('!important is not supported')
    }

    this._values.set(propertyName, value)
    this._onChange(propertyName, value)
  }

  removeProperty(propertyName: string): string {
    const prev = this._values.get(propertyName)

    this._values.delete(propertyName)
    this._onChange(propertyName, undefined)

    return prev ?? ''
  }

  get cssText(): string {
    return Array.from(this._values).map(([prop, value]) => `${prop}: ${value}`).join('; ')
  }

  set cssText(cssText: string) {
    for (const prop of this._values.keys()) {
      this.removeProperty(prop)
    }

    for (const r of parseRules(`dummy { ${cssText} }`)) {
      for (const [prop, v] of Object.entries(r.props)) {
        this.setProperty(prop, v)
      }
    }
  }

  // TODO: extend Array<string> but it's tricky because it should be
  // lowercased and it should also expand shorthands and almost nobody uses it
  // so it will probably just remain unsupported
  [index: number]: string

  get length() {
    return UNSUPPORTED()
  }

  item(index: number): string {
    return UNSUPPORTED()
  }

  get alignContent() { return this.getPropertyValue('align-content') }
  set alignContent(v: string) { this.setProperty('align-content', v) }
  get alignItems() { return this.getPropertyValue('align-items') }
  set alignItems(v: string) { this.setProperty('align-items', v) }
  get alignSelf() { return this.getPropertyValue('align-self') }
  set alignSelf(v: string) { this.setProperty('align-self', v) }
  get background() { return this.getPropertyValue('background') }
  set background(v: string) { this.setProperty('background', v) }
  get backgroundColor() { return this.getPropertyValue('background-color') }
  set backgroundColor(v: string) { this.setProperty('background-color', v) }
  get border() { return this.getPropertyValue('border') }
  set border(v: string) { this.setProperty('border', v) }
  get borderBottom() { return this.getPropertyValue('border-bottom') }
  set borderBottom(v: string) { this.setProperty('border-bottom', v) }
  get borderBottomColor() { return this.getPropertyValue('border-bottom-color') }
  set borderBottomColor(v: string) { this.setProperty('border-bottom-color', v) }
  get borderBottomLeftRadius() { return this.getPropertyValue('border-bottom-left-radius') }
  set borderBottomLeftRadius(v: string) { this.setProperty('border-bottom-left-radius', v) }
  get borderBottomRightRadius() { return this.getPropertyValue('border-bottom-right-radius') }
  set borderBottomRightRadius(v: string) { this.setProperty('border-bottom-right-radius', v) }
  get borderBottomStyle() { return this.getPropertyValue('border-bottom-style') }
  set borderBottomStyle(v: string) { this.setProperty('border-bottom-style', v) }
  get borderBottomWidth() { return this.getPropertyValue('border-bottom-width') }
  set borderBottomWidth(v: string) { this.setProperty('border-bottom-width', v) }
  get borderColor() { return this.getPropertyValue('border-color') }
  set borderColor(v: string) { this.setProperty('border-color', v) }
  get borderLeft() { return this.getPropertyValue('border-left') }
  set borderLeft(v: string) { this.setProperty('border-left', v) }
  get borderLeftColor() { return this.getPropertyValue('border-left-color') }
  set borderLeftColor(v: string) { this.setProperty('border-left-color', v) }
  get borderLeftStyle() { return this.getPropertyValue('border-left-style') }
  set borderLeftStyle(v: string) { this.setProperty('border-left-style', v) }
  get borderLeftWidth() { return this.getPropertyValue('border-left-width') }
  set borderLeftWidth(v: string) { this.setProperty('border-left-width', v) }
  get borderRadius() { return this.getPropertyValue('border-radius') }
  set borderRadius(v: string) { this.setProperty('border-radius', v) }
  get borderRight() { return this.getPropertyValue('border-right') }
  set borderRight(v: string) { this.setProperty('border-right', v) }
  get borderRightColor() { return this.getPropertyValue('border-right-color') }
  set borderRightColor(v: string) { this.setProperty('border-right-color', v) }
  get borderRightStyle() { return this.getPropertyValue('border-right-style') }
  set borderRightStyle(v: string) { this.setProperty('border-right-style', v) }
  get borderRightWidth() { return this.getPropertyValue('border-right-width') }
  set borderRightWidth(v: string) { this.setProperty('border-right-width', v) }
  get borderStyle() { return this.getPropertyValue('border-style') }
  set borderStyle(v: string) { this.setProperty('border-style', v) }
  get borderTop() { return this.getPropertyValue('border-top') }
  set borderTop(v: string) { this.setProperty('border-top', v) }
  get borderTopColor() { return this.getPropertyValue('border-top-color') }
  set borderTopColor(v: string) { this.setProperty('border-top-color', v) }
  get borderTopLeftRadius() { return this.getPropertyValue('border-top-left-radius') }
  set borderTopLeftRadius(v: string) { this.setProperty('border-top-left-radius', v) }
  get borderTopRightRadius() { return this.getPropertyValue('border-top-right-radius') }
  set borderTopRightRadius(v: string) { this.setProperty('border-top-right-radius', v) }
  get borderTopStyle() { return this.getPropertyValue('border-top-style') }
  set borderTopStyle(v: string) { this.setProperty('border-top-style', v) }
  get borderTopWidth() { return this.getPropertyValue('border-top-width') }
  set borderTopWidth(v: string) { this.setProperty('border-top-width', v) }
  get borderWidth() { return this.getPropertyValue('border-width') }
  set borderWidth(v: string) { this.setProperty('border-width', v) }
  get bottom() { return this.getPropertyValue('bottom') }
  set bottom(v: string) { this.setProperty('bottom', v) }
  get boxShadow() { return this.getPropertyValue('box-shadow') }
  set boxShadow(v: string) { this.setProperty('box-shadow', v) }
  get color() { return this.getPropertyValue('color') }
  set color(v: string) { this.setProperty('color', v) }
  get display() { return this.getPropertyValue('display') }
  set display(v: string) { this.setProperty('display', v) }
  get flex() { return this.getPropertyValue('flex') }
  set flex(v: string) { this.setProperty('flex', v) }
  get flexBasis() { return this.getPropertyValue('flex-basis') }
  set flexBasis(v: string) { this.setProperty('flex-basis', v) }
  get flexDirection() { return this.getPropertyValue('flex-direction') }
  set flexDirection(v: string) { this.setProperty('flex-direction', v) }
  get flexFlow() { return this.getPropertyValue('flex-flow') }
  set flexFlow(v: string) { this.setProperty('flex-flow', v) }
  get flexGrow() { return this.getPropertyValue('flex-grow') }
  set flexGrow(v: string) { this.setProperty('flex-grow', v) }
  get flexShrink() { return this.getPropertyValue('flex-shrink') }
  set flexShrink(v: string) { this.setProperty('flex-shrink', v) }
  get flexWrap() { return this.getPropertyValue('flex-wrap') }
  set flexWrap(v: string) { this.setProperty('flex-wrap', v) }
  get font() { return this.getPropertyValue('font') }
  set font(v: string) { this.setProperty('font', v) }
  get fontFamily() { return this.getPropertyValue('font-family') }
  set fontFamily(v: string) { this.setProperty('font-family', v) }
  get fontSize() { return this.getPropertyValue('font-size') }
  set fontSize(v: string) { this.setProperty('font-size', v) }
  get fontStyle() { return this.getPropertyValue('font-style') }
  set fontStyle(v: string) { this.setProperty('font-style', v) }
  get fontVariant() { return this.getPropertyValue('font-variant') }
  set fontVariant(v: string) { this.setProperty('font-variant', v) }
  get fontWeight() { return this.getPropertyValue('font-weight') }
  set fontWeight(v: string) { this.setProperty('font-weight', v) }
  get height() { return this.getPropertyValue('height') }
  set height(v: string) { this.setProperty('height', v) }
  get justifyContent() { return this.getPropertyValue('justify-content') }
  set justifyContent(v: string) { this.setProperty('justify-content', v) }
  get left() { return this.getPropertyValue('left') }
  set left(v: string) { this.setProperty('left', v) }
  get lineHeight() { return this.getPropertyValue('line-height') }
  set lineHeight(v: string) { this.setProperty('line-height', v) }
  get margin() { return this.getPropertyValue('margin') }
  set margin(v: string) { this.setProperty('margin', v) }
  get marginBottom() { return this.getPropertyValue('margin-bottom') }
  set marginBottom(v: string) { this.setProperty('margin-bottom', v) }
  get marginLeft() { return this.getPropertyValue('margin-left') }
  set marginLeft(v: string) { this.setProperty('margin-left', v) }
  get marginRight() { return this.getPropertyValue('margin-right') }
  set marginRight(v: string) { this.setProperty('margin-right', v) }
  get marginTop() { return this.getPropertyValue('margin-top') }
  set marginTop(v: string) { this.setProperty('margin-top', v) }
  get maxHeight() { return this.getPropertyValue('max-height') }
  set maxHeight(v: string) { this.setProperty('max-height', v) }
  get maxWidth() { return this.getPropertyValue('max-width') }
  set maxWidth(v: string) { this.setProperty('max-width', v) }
  get minHeight() { return this.getPropertyValue('min-height') }
  set minHeight(v: string) { this.setProperty('min-height', v) }
  get minWidth() { return this.getPropertyValue('min-width') }
  set minWidth(v: string) { this.setProperty('min-width', v) }
  get overflow() { return this.getPropertyValue('overflow') }
  set overflow(v: string) { this.setProperty('overflow', v) }
  get padding() { return this.getPropertyValue('padding') }
  set padding(v: string) { this.setProperty('padding', v) }
  get paddingBottom() { return this.getPropertyValue('padding-bottom') }
  set paddingBottom(v: string) { this.setProperty('padding-bottom', v) }
  get paddingLeft() { return this.getPropertyValue('padding-left') }
  set paddingLeft(v: string) { this.setProperty('padding-left', v) }
  get paddingRight() { return this.getPropertyValue('padding-right') }
  set paddingRight(v: string) { this.setProperty('padding-right', v) }
  get paddingTop() { return this.getPropertyValue('padding-top') }
  set paddingTop(v: string) { this.setProperty('padding-top', v) }
  get right() { return this.getPropertyValue('right') }
  set right(v: string) { this.setProperty('right', v) }
  get textAlign() { return this.getPropertyValue('text-align') }
  set textAlign(v: string) { this.setProperty('text-align', v) }
  get top() { return this.getPropertyValue('top') }
  set top(v: string) { this.setProperty('top', v) }
  get transform() { return this.getPropertyValue('transform') }
  set transform(v: string) { this.setProperty('transform', v) }
  get width() { return this.getPropertyValue('width') }
  set width(v: string) { this.setProperty('width', v) }

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
