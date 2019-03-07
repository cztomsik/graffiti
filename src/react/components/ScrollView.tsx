import * as React from 'react'
import { ScrollViewProps } from '../react-native-types'

// TODO
const ScrollView = (props: ScrollViewProps & { children? }) => {
  return (
    <host-surface flex={{ flexGrow: 1, flexShrink: 1, flexBasis: { tag: 'Auto' }}} backgroundColor={[0, 0, 0, 1]}>
      {props.children}
    </host-surface>
  )
}

export default ScrollView
