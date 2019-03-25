import * as React from 'react'
import { TextProps } from '../react-native-types'
import { parseColor } from '../../core/utils'
import { TextAlign } from '../../core/generated';
import StyleSheet from '../Stylesheet';

export function Text({ style = {}, children = [] }: TextProps) {
  /*const {
    _props,
    fontSize = 16,
    color = '#000000',
    lineHeight = 30,
    textAlign = 'left'
  } = StyleSheet.flatten(style) as any

  return (
    <host-surface
      text={{
        fontSize,
        color: parseColor(color),
        lineHeight,
        align: TEXT_ALIGN[textAlign],
        text: [].concat(children).filter(numberOrString).join('')
      }}
      {..._props}
      size={SIZE_AUTO}
    />
  )*/

  const View = 'View'
  return <View style={{ width: 100, height: 30 }} />
}

function numberOrString(v) {
  return typeof v === 'string' || typeof v === 'number';
}

const TEXT_ALIGN = {
  left: TextAlign.Left,
  center: TextAlign.Center,
  right: TextAlign.Right
}

const SIZE_AUTO = [{ tag: 'Auto' }, { tag: 'Auto' }]
