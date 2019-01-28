import * as React from 'react'
import { StyleSheet } from '..'
import {
  ImageProps,
  ImageSourcePropType,
  ImageURISource
} from '../react-native-types'
import { N, NativeApi } from '../../core/nativeApi'
import { ResourceManager } from '../..'
import { RenderOp } from '../../core/RenderOperation'
import { WINDOW_HACK } from '../../core/Window'

// TODO fix it!
const native: NativeApi = require('../../../native')

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

  // TODO this is super ugly
  // also sideeffectful
  let existing = allRegisteredImages.get(source.uri)

  if (existing === undefined) {
    existing = genImageId()

    allRegisteredImages.set(source.uri, existing)

    console.log('registering image for ' + source.uri, WINDOW_HACK.ref)

    native.registerImage(
      WINDOW_HACK.ref,
      JSON.stringify({ id: existing, size: 64 })
    )
    console.log('finished registering ' + source.uri)
  }

  const imgBrush = React.useMemo(
    () => ResourceManager.createBrush([RenderOp.Image(existing)]),
    [existing]
  )

  return (
    <host-surface clip={clip} brush={brush} layout={layout}>
      <host-image imgBrush={imgBrush} />
    </host-surface>
  )
}

// TODO proper image loading

const genImageId = (() => {
  let id = 1

  return (): N.ImageId => id++ as any
})()

const allRegisteredImages = new Map<string, N.ImageId>()
