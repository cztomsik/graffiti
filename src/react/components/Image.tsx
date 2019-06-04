import * as React from 'react'
import View from './View';
import {
  ImageProps,
  ImageSourcePropType,
  ImageURISource
} from 'react-native'

export function Image(props: ImageProps) {
  if (!isUriSource(props.source)) {
    throw new Error('we only support uri sources so far')
  }

  return (
    <View style={[props.style, { backgroundImageUrl: props.source.uri }]} />
  )
}

const isUriSource = (src: ImageSourcePropType): src is ImageURISource => {
  const uriSrc = src as ImageURISource
  return !!uriSrc.uri
}
