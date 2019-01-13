import * as yoga from 'yoga-layout'
import { remove } from './utils'
import { HasLayout, Container, DrawBrushFunction } from './types'
import { ResourceManager } from '.'
import { BridgeRect, BridgeBrush, BridgeClip } from './ResourceManager'
import { RenderOp } from './RenderOperation'

const native = require('../../native')

// container with a layout and an optional brush
// children should have a layout too
class Surface extends native.Surface {
  update({ brush = undefined, clip = undefined, layout = DEFAULT_LAYOUT }) {
    // TODO: not sure why it raises TypeError for undefined values
    // TypeError: failed downcast to RcOpResource
    // it's weird because everything went fine (false negative)
    try {
      super.update(brush, clip, layout)
    } catch (e) {
      if ( ! (e instanceof TypeError)) {
        throw e
      }
    }
  }

}

// TODO: this should be just value (it's a function until ResourceManager gets really separated)
const DEFAULT_LAYOUT = ResourceManager.getLayout({})

export default Surface
