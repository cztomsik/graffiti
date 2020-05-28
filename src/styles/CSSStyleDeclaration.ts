import { Node } from '../nodes/Node'
import { Element } from '../nodes/Element'
import { updateText } from '../nodes/Text'
import { UNSUPPORTED } from '../util'

// minimal impl just to get something working
//
// for now, only setters are supported
// (and many props are missing)
export class CSSStyleDeclaration implements globalThis.CSSStyleDeclaration {
  private _elementId

  constructor(el: Element) {
    this._elementId = el._nativeId
  }

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
    return UNSUPPORTED()
    //this.setProperty(prop, defaults[prop])
    //return ''
  }

  // note we expect only valid names (there's no normalization, error reporting, etc.)
  // but that shouldn't be a problem if you're using modern IDE and/or tooling
  setProperty(prop: string, v?: string | null, priority?: string | null): void {
    return UNSUPPORTED()

    // TODO: put prop in this[length++]
    //       and value in this.values
    //       and push new style attr

    /*
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
    */
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
