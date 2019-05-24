import * as React from 'react'
import { TextProps } from '../react-native-types'
import { parseColor } from '../../core/utils'
import { TextAlign } from '../../core/generated';
import StyleSheet from '../Stylesheet';
import View from './View'

// we might make it native (host) comp in the future but for now we want to
// avoid having to mess with `createTextInstance()`, so anything inside <Text>
// actually gets joined to one string and passed to View style
export function Text({ children = [], style = {}, ...rest }: TextProps) {
  const content = [].concat(children).filter(numberOrString).join('')

  return (
    <View
      style={[style, { content }]}
      {...rest}
    />
  )
}

function numberOrString(v) {
  return typeof v === 'string' || typeof v === 'number';
}
