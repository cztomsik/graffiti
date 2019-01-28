import * as React from 'react'
import { StyleSheet } from '..'
import { ViewProps } from '../react-native-types'

const View: React.SFC<ViewProps> = ({ style = {}, children }) => {
  const { _brush: brush, _layout: layout, _clip: clip } = StyleSheet.flatten(
    style
  ) as any

  return <host-surface {...{ brush, layout, clip }}>{children}</host-surface>
}

export default View
