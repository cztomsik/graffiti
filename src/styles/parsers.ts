import * as assert from 'assert'
import { Align, Display, Overflow, TextAlign, FlexDirection, FlexWrap, Dimension, Transform } from '../core/nativeApi'

// should be represented as Smi (small integer) in V8 and so the comparison
// should be very fast and without need for heap lookup
// also, for bare enums this will be monomorphic
export const INVALID = -1
type Invalid = typeof INVALID

export function parseDisplay(value: string): Display | Invalid {
  switch (value) {
    case 'none': return Display.None
    case 'block': return Display.Block
    case 'flex': return Display.Flex
  }

  return INVALID
}

export function parseOverflow(value: string): Overflow | Invalid {
  switch (value) {
    case 'hidden': return Overflow.Hidden
    case 'scroll': return Overflow.Scroll
    case 'visible': return Overflow.Visible
  }

  return INVALID
}

export function parseFlexWrap(value: string): FlexWrap | Invalid {
  switch (value) {
    case 'wrap': return FlexWrap.Wrap
    case 'wrap-reverse': return FlexWrap.WrapReverse
    case 'nowrap': return FlexWrap.NoWrap
  }

  return INVALID
}

export function parseFlexDirection(value: string): FlexDirection | Invalid {
  switch (value) {
    case 'column': return FlexDirection.Column
    case 'row': return FlexDirection.Row
    case 'column-reverse': return FlexDirection.ColumnReverse
    case 'row-reverse': return FlexDirection.RowReverse
  }

  return INVALID
}

export function parseAlign(value: string): Align | Invalid {
  switch (value) {
    case 'center': return Align.Center
    case 'auto': return Align.Auto
    case 'flex-end': return Align.FlexEnd
    case 'flex-start': return Align.FlexStart
    case 'space-around': return Align.SpaceAround
    case 'space-between': return Align.SpaceBetween
    case 'space-evenly': return Align.SpaceEvenly
    case 'stretch': return Align.Stretch
    case 'baseline': return Align.Baseline
  }

  return INVALID
}

// TODO: invalid value should not throw, nor send NaN
export const parseDimension = (value?: string | number) => {
  if (value === undefined) {
    return Dimension.Undefined()
  }

  value = '' + value

  if (value.endsWith('%')) {
    return Dimension.Percent(parseFloat(value))
  }

  if (value === 'auto') {
    return Dimension.Auto()
  }

  return Dimension.Px(parseFloat(value))
}

// TODO: put it to proper testcases
// TODO: consider Option<Dimension>
assert.deepEqual(parseDimension(undefined), Dimension.Undefined())
assert.deepEqual(parseDimension('auto'), Dimension.Auto())
assert.deepEqual(parseDimension('100%'), Dimension.Percent(100))
assert.deepEqual(parseDimension('10px'), Dimension.Px(10))

export const parseTransform = (v) => {
  let match

  if (match = v.match(/scale\(([\d\.\s]+)(?:,([\d\.\s]+))?\)/)) {
    const [, x, y = x] = match
    return Transform.Scale(parseFloat(x), parseFloat(y))
  }

  return undefined
}


// TODO: rgb(), rgba()
// https://docs.rs/crate/cssparser/0.25.3/source/src/color.rs
export const parseColor = (str: string) => {
  if (!str) {
    return undefined
  }

  if (str[0] === '#') {
    return parseHashColor(str.slice(1))
  }

  console.warn(`only colors starting with # are supported (got ${JSON.stringify(str)})`)
  return INVALID
}

// note that in rgba(xx, xx, xx, x), alpha is 0-1
export const parseHashColor = (str: string) => {
  let a = 255

  switch (str.length) {
    // rgba
    case 8:
      a = parseHex(str.slice(7, 9))

    case 6:
      return [parseHex(str.slice(0, 2)), parseHex(str.slice(2, 4)), parseHex(str.slice(4, 6)), a]

    // short alpha
    case 4:
      a = parseHex(str.slice(3, 4)) * 17

    // short
    case 3:
      return [parseHex(str.slice(0, 1)) * 17, parseHex(str.slice(1, 2)) * 17, parseHex(str.slice(2, 3)) * 17, a]

    default:
      throw new Error(`invalid color #${str}`)
  }
}

export const parseHex = (str: string) => parseInt(str, 16)
