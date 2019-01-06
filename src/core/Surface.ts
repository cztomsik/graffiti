import * as yoga from 'yoga-layout'
import { remove } from './utils'
import { HasLayout, Container, DrawBrushFunction } from './types'
import { ResourceManager } from '.'
import { BridgeRect, BucketId } from './ResourceManager'
import { RenderOp } from './RenderOperation'

// container with a layout and an optional brush
// children should have a layout too
class Surface implements Container<HasLayout>, HasLayout {
  yogaNode = yoga.Node.create()
  children = []
  brush?: BucketId[]
  clip?: BucketId[]

  appendChild(child) {
    this.insertAt(child, this.children.length)
  }

  insertBefore(child, before) {
    this.insertAt(child, this.children.indexOf(before))
  }

  insertAt(child, index) {
    this.children.splice(index, 0, child)
    this.yogaNode.insertChild(child.yogaNode, index)
  }

  removeChild(child) {
    remove(this.children, child)
    this.yogaNode.removeChild(child.yogaNode)
  }

  update({ brush = undefined, clip = undefined, layout = DEFAULT_LAYOUT }) {
    this.brush = brush
    this.clip = clip
    updateYogaNode(this.yogaNode, layout)
  }

  write(drawBrush: DrawBrushFunction, x, y) {
    const { left, top, width, height } = this.yogaNode.getComputedLayout()
    const rect: BridgeRect = [left + x, top + y, width, height]

    // TODO: translate stacking context
    // updating layout[0] a layout[1] should be enough

    if (this.clip !== undefined) {
      drawBrush(this.clip, rect)
    }

    if (this.brush !== undefined) {
      drawBrush(this.brush, rect)
    }

    this.children.forEach(c => c.write(drawBrush, rect[0], rect[1]))

    // TODO: pop stacking context

    if (this.clip !== undefined) {
      drawBrush(POP_CLIP, [0, 0, 0, 0])
    }
  }
}

// TODO: should be in the same "land" with yoga so there is no hopping back and forth)
const updateYogaNode = (n: yoga.YogaNode, props: any) => {
  const p = props

  n.setWidth(p.width)
  n.setHeight(p.height)

  n.setAlignContent(p.alignContent)
  n.setAlignItems(p.alignItems)
  n.setAlignSelf(p.alignSelf)
  n.setJustifyContent(p.justifyContent)

  n.setFlexBasis(p.flexBasis)
  n.setFlexGrow(p.flexGrow)
  n.setFlexShrink(p.flexShrink)
  n.setFlexWrap(p.flexWrap)
  n.setFlexDirection(p.flexDirection)

  n.setMargin(yoga.EDGE_TOP, p.marginTop)
  n.setMargin(yoga.EDGE_RIGHT, p.marginRight)
  n.setMargin(yoga.EDGE_BOTTOM, p.marginBottom)
  n.setMargin(yoga.EDGE_LEFT, p.marginLeft)

  n.setPadding(yoga.EDGE_TOP, p.paddingTop)
  n.setPadding(yoga.EDGE_RIGHT, p.paddingRight)
  n.setPadding(yoga.EDGE_BOTTOM, p.paddingBottom)
  n.setPadding(yoga.EDGE_LEFT, p.paddingLeft)

  n.setOverflow(p.overflow)
}

// TODO: this should be just value (it's a function until ResourceManager gets really separated)
const DEFAULT_LAYOUT = ResourceManager.getLayout({})
const POP_CLIP = [ResourceManager.createBucket(RenderOp.PopClip())]

export default Surface
