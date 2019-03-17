import * as React from 'react'
import { StyleSheet, View } from '..'
import { TextProps } from '../react-native-types'
import { parseColor } from '../../core/utils'
import { TextAlign } from '../../core/generated';

export function Text({ style = {}, children = [] }: TextProps) {
  const {
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
      size={[{ tag: 'Auto' }, { tag: 'Auto' }]}
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
