import * as React from 'react'
import StyleSheet from '../Stylesheet';
import { ViewProps } from '../react-native-types'

const View: React.SFC<ViewProps> = ({ style = {}, children }) => {
  const { _props } = StyleSheet.flatten(
    style
  ) as any

  return <host-surface {..._props}>{children}</host-surface>
}

export default View
