import * as React from 'react'
import { StyleSheet } from '..'
import {
  ImageProps,
  ImageSourcePropType,
  ImageURISource
} from '../react-native-types'

const isUriSource = (src: ImageSourcePropType): src is ImageURISource => {
  const uriSrc = src as ImageURISource
  return !!uriSrc.uri
}

export const Image: React.SFC<ImageProps> = ({ style = {}, source }) => {
  const { _brush: brush, _layout: layout, _clip: clip } = StyleSheet.flatten(
    style
  ) as any

  if (!isUriSource(source)) {
    throw new Error('we only support uri sources so far')
  }

  // TODO
  return (
    <host-surface size={[{ tag: 'Point', value: 100 }, { tag: 'Point', value: 20 }]} text={{ fontSize: 16, color: [0, 0, 0, 0], lineHeight: 16, text: 'TODO: Image' }} />
  )
}
