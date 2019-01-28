import { Container } from './types'
import { ResourceManager, Surface } from '.'
import { NativeApi, N } from './nativeApi'
import { HostImageProps } from '../react/reconciler'

const native: NativeApi = require('../../native')

// container with a layout and an optional brush
// children should have a layout too
export class Img implements Container<Surface> {
  ref: N.Surface

  constructor() {
    this.ref = native.surface_create()
  }

  appendChild(_child: Surface) {
    throw new Error('children in images are not supported')
  }

  insertBefore(_child: Surface, _before: Surface) {
    throw new Error('children in images are not supported')
  }

  removeChild(_child: Surface) {
    throw new Error('children in images are not supported')
  }

  update({ imgBrush }: HostImageProps) {
    native.surface_update(this.ref, imgBrush, undefined, FILL_SPACE)
  }
}

const FILL_SPACE = ResourceManager.getLayout({ height: '100%', width: '100%' })
