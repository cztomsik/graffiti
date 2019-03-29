import * as React from 'react'
import { TextProps } from '../react-native-types'
import { parseColor } from '../../core/utils'
import { TextAlign } from '../../core/generated';
import StyleSheet from '../Stylesheet';
import View from './View'

// we might make it native (host) comp in the future but for now we want to
// avoid having to mess with `createTextInstance()`, so anything inside <Text>
// actually gets joined to one string and passed to View via internal `_text`
export function Text({ children = [], ...rest }: TextProps) {
  const {
    _props,
    fontSize = 16,
    color = '#000000',
    lineHeight = 30,
    textAlign = 'left'
  } = StyleSheet.flatten(rest.style || {}) as any

  // TODO: textContent + (fontFamily, color, ...) in style?
  return (
    <View
      _text={{
        fontSize,
        color: parseColor(color),
        lineHeight,
        align: TEXT_ALIGN[textAlign],
        text: [].concat(children).filter(numberOrString).join('')
      }}
      {...rest}
    />
  )
}

function numberOrString(v) {
  return typeof v === 'string' || typeof v === 'number';
}

const TEXT_ALIGN = {
  left: TextAlign.Left,
  center: TextAlign.Center,
  right: TextAlign.Right
}
