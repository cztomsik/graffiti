import * as React from 'react'
import { StyleSheet, View } from '..'
import { TextProps } from '../react-native-types'
import { parseColor } from '../../core/utils'

export function Text({ style = {}, children = [] }: TextProps) {
  const {
    _props,
    fontSize = 16,
    color = '#000000',
    lineHeight = 30
  } = StyleSheet.flatten(style) as any

  return (
    <host-surface
      text={{
        fontSize,
        color: parseColor(color),
        lineHeight,
        text: [].concat(children).join('')
      }}
      {..._props}
      size={[{ tag: 'Point', value: 100 }, { tag: 'Point', value: 30 }]}
    />
  )
}
