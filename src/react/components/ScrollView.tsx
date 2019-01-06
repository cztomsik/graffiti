import * as React from 'react'
import { ResourceManager } from '../..'
import { ScrollViewProps } from '../react-native-types'
import { View } from '..'
import { __callbacks } from '../../core/Window'
import { RenderOp } from '../../core/RenderOperation'

const ScrollView = (props: ScrollViewProps & { children? }) => (
  <host-surface brush={SAVE_RECT} layout={SCROLL_LAYOUT}>
    <host-surface clip={SCROLL_CLIP} brush={SCROLL_TEST}>
      {props.children}
    </host-surface>
  </host-surface>
)

const SAVE_RECT = [ResourceManager.createBucket(RenderOp.SaveRect())]
const SCROLL_CLIP = [ResourceManager.createBucket(RenderOp.PushScrollClip())]
const SCROLL_LAYOUT = ResourceManager.getLayout({ flex: 1, overflow: 'scroll' })
const SCROLL_CB = __callbacks.push(() => {}) - 1
const SCROLL_TEST = [ResourceManager.createBucket(RenderOp.HitTest(SCROLL_CB))]

export default ScrollView
