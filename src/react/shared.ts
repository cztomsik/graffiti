import * as yoga from 'yoga-layout'
import { remove } from './utils'

// base node with layout and children
// - appendChild/insertBefore/removeChild/update() is needed for all react "instances"
// - and we need yogaNode and write() for anything what can be root
abstract class Base<T> {
  yogaNode = yoga.Node.create()
  children: T[] = []

  insertAt(c, index) {
    this.children.splice(index, 0, c)
  }

  removeChild(c) {
    remove(this.children, c)
  }

  appendChild(c) {
    this.insertAt(c, this.children.length)
  }

  insertBefore(c, before) {
    const index = this.children.indexOf(before)

    this.insertAt(index, c)
  }

  getLayout(parentLeft, parentTop) {
    const { left, top, width, height } = this.yogaNode.getComputedLayout()
    return [parentLeft + left, parentTop + top, width, height]
  }

  abstract update(props, window)
  abstract write(bucketIds, layouts, x, y)
}

export class View extends Base<View | Text> {
  backgroundBucketId
  borderBucketId

  insertAt(c, index) {
    super.insertAt(c, index)
    this.yogaNode.insertChild(c.yogaNode, index)
  }

  removeChild(c) {
    remove(this.children, c)
    this.yogaNode.removeChild(c.yogaNode)
  }

  update({ layout, background, border }, window) {
    updateYogaNode(this.yogaNode, layout)

    this.backgroundBucketId = window._setBucket(this.backgroundBucketId, background)
    this.borderBucketId = window._setBucket(this.borderBucketId, border)
  }

  write(bucketIds, layouts, x, y) {
    const layout = this.getLayout(x, y)

    if (this.backgroundBucketId !== undefined) {
      bucketIds.push(this.backgroundBucketId)
      layouts.push(layout)
    }

    if (this.borderBucketId !== undefined) {
      bucketIds.push(this.borderBucketId)
      layouts.push(layout)
    }

    this.children.forEach(c => c.write(bucketIds, layouts, layout[0], layout[1]))
  }
}

export class Text extends Base<TextNode> {
  content
  breaks
  fontInstanceKey
  lineHeight
  color
  bucketId
  glyphs = []
  contentWidth
  contentHeight

  constructor(private window) {
    super()

    this.yogaNode.setMeasureFunc(((width) => {
      this.updateGlyphs(width)

      return { width: this.contentWidth, height: this.contentHeight }
    }) as any)
  }

  insertAt(c, index) {
    super.insertAt(c, index)
    c.text = this
    this.updateContent()
  }

  removeChild(c) {
    super.appendChild(c)
    c.text = undefined
    this.updateContent()
  }

  update({ fontInstanceKey = [1, 2], color, lineHeight }, window) {
    this.fontInstanceKey = fontInstanceKey
    this.color = color
    this.lineHeight = lineHeight

    this.updateBucket()
  }

  updateContent() {
    this.content = this.children.map(c => c.value).join('')
    this.breaks = parseBreaks(this.content)
    this.yogaNode.markDirty()
  }

  updateBucket() {
    this.bucketId = this.window._setBucket(this.bucketId, {
      Text: [{
        font_key: this.fontInstanceKey,
        color: this.color
      }, this.glyphs]
    })
  }

  updateGlyphs(maxWidth) {
    const [indices, advances] = this.window.getGlyphIndicesAndAdvances(this.content)
    let x = 0
    const xs = [0, ...advances.map(a => x += a)]

    const lines = []

    // do the word-wrap and figure out "line slices"
    {
      let tokenStart = 0
      let lineStart = tokenStart
      let nextBreak = maxWidth

      for (const tokenEnd of this.breaks) {
        const ch = this.content[tokenStart]

        if (ch === ' ') {
          tokenStart = tokenEnd
          continue
        }

        if (ch === '\n') {
          lines.push([lineStart, tokenStart])
          lineStart = tokenEnd
          tokenStart = tokenEnd
          nextBreak = xs[tokenEnd] + maxWidth
          continue
        }

        // not exactly (glyph can be shorter than its advance) but it's probably not worth memory and cpu
        if (xs[tokenEnd] > nextBreak) {
          lines.push([lineStart, tokenStart])
          lineStart = tokenStart
          nextBreak = xs[tokenStart] + maxWidth
        }

        // after last wrap check
        if (tokenEnd === this.content.length) {
          lines.push([lineStart, this.content.length])
          break;
        }

        tokenStart = tokenEnd
      }
    }

    const glyphs = []

    // layout lines
    for (const [lineIndex, [start, end]] of lines.entries()) {
      // TODO: text-align
      let x = 0

      // TODO: font metrics
      let y = this.lineHeight * (lineIndex + 0.7)

      for (let i = start; i < end; i++) {
        glyphs.push([indices[i], [x, y]])
        x += advances[i]
      }
    }

    // finish
    this.glyphs = glyphs
    this.contentWidth = lines.length ?xs[lines[0][1]] :200
    this.contentHeight = lines.length * this.lineHeight

    this.updateBucket()
  }

  write(bucketIds, layouts, x, y) {
    const layout = this.getLayout(x, y)

    // TODO: we don't have proper font-metrics yet so we need to extend the box for now
    layout[3] += this.lineHeight

    bucketIds.push(this.window._consts.TEXT_STACKING_CONTEXT)
    layouts.push(layout)

    bucketIds.push(this.bucketId)
    layouts.push([0, 0, layout[2], layout[3]])

    bucketIds.push(this.window._consts.POP_STACKING_CONTEXT)
    layouts.push([0, 0, 0, 0])
  }
}

export class TextNode {
  value
  text

  constructor(value) {
    this.setValue(value)
  }

  setValue(value) {
    this.value = value

    if (this.text !== undefined) {
      this.text.updateContent()
    }
  }
}


// TODO: should be in the same "land" with yoga so there is no hopping back and forth)
// takes array of numbers
function updateYogaNode(n: yoga.YogaNode, values) {
  let i = 0
  const v = values

  n.setWidth(v[i++])
  n.setHeight(v[i++])
  n.setFlexDirection(v[i++])
  n.setFlex(v[i++])
  // n.setFlexGrow(v[i++])
  // n.setFlexShrink(v[i++])

  n.setMargin(yoga.EDGE_TOP, v[i++])
  n.setMargin(yoga.EDGE_RIGHT, v[i++])
  n.setMargin(yoga.EDGE_BOTTOM, v[i++])
  n.setMargin(yoga.EDGE_LEFT, v[i++])

  n.setPadding(yoga.EDGE_TOP, v[i++])
  n.setPadding(yoga.EDGE_RIGHT, v[i++])
  n.setPadding(yoga.EDGE_BOTTOM, v[i++])
  n.setPadding(yoga.EDGE_LEFT, v[i++])

  n.setAlignContent(v[i++])
  n.setAlignItems(v[i++])
  n.setAlignSelf(v[i++])
  n.setJustifyContent(v[i++])
}


const TOKEN_REGEX = /[^\n ]+|\n| +/g

const parseBreaks = str => {
  let i = 0

  return str.match(TOKEN_REGEX).map(t => i += t.length)
}
