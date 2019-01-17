import { Container } from './types'
import { remove } from './utils'
import { TextPart, ResourceManager, Surface } from '.'
import {
  RenderOp,
  TransformStyle,
  MixBlendMode,
  BridgeColor,
  GlyphInstance
} from './RenderOperation'
import { BridgeBrush } from './ResourceManager'

const native = require('../../native')

// TODO: this should be (at least partially) in rust
export class TextContainer {
  ref
  children = []
  content = ''
  breaks = []
  fontInstanceKey: [number, number] = [1, 0]
  lineHeight
  color: BridgeColor
  brush?: BridgeBrush
  glyphs: GlyphInstance[] = []
  contentWidth
  contentHeight

  constructor() {
    this.ref = native.surface_create()

    // note that yoga will not call this if width & height is fixed!
    native.surface_set_measure_func(this.ref, width => {
      this.updateGlyphs(width)

      return { width: this.contentWidth, height: this.contentHeight }
    })
  }

  appendChild(child) {
    this.children.push(child)
    child.textContainer = this
    this.updateContent()
  }

  insertBefore(child, before) {
    this.children.splice(this.children.indexOf(before), 0, child)
    child.textContainer = this
    this.updateContent()
  }

  removeChild(child) {
    remove(this.children, child)
    child.textContainer = undefined
    this.updateContent()
  }

  update({ fontSize = 16, color, lineHeight }) {
    if (
      fontSize === this.fontInstanceKey[1] &&
      color === this.color &&
      lineHeight === this.lineHeight
    ) {
      return
    }

    // TODO: support any size
    this.fontInstanceKey = [1, fontSize]
    this.color = color
    this.lineHeight = lineHeight

    this.updateBrush()
  }

  updateContent() {
    this.content = this.children.map(c => c.value).join('')
    this.breaks = parseBreaks(this.content)
    native.surface_mark_dirty(this.ref)
  }

  updateBrush() {
    const brush = ResourceManager.createBrush([
      RenderOp.Text(
        { color: this.color, font_key: this.fontInstanceKey },
        this.glyphs
      )
    ])

    Surface.prototype.update.call(this, { brush })
  }

  updateGlyphs(maxWidth) {
    const [indices, advances] = ResourceManager.getGlyphIndicesAndAdvances(
      this.fontInstanceKey[1],
      this.content
    )
    let x = 0
    const xs = [0, ...(advances as Float32Array).map(a => (x += a))]

    const lines = []

    // do the word-wrap and figure out "line slices"
    {
      let tokenStart = 0
      let lineStart = tokenStart
      let nextBreak = maxWidth

      for (const tokenEnd of this.breaks) {
        const ch = this.content[tokenStart]

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
          break
        }

        if (ch === ' ') {
          tokenStart = tokenEnd
          continue
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
    this.contentWidth = lines.length ? xs[lines[0][1]] : 200
    this.contentHeight = lines.length * this.lineHeight

    this.updateBrush()
  }
}

const TOKEN_REGEX = /[^\n ]+|\n| +/g

const parseBreaks = str => {
  if (str === '') {
    return []
  }

  let i = 0

  return str.match(TOKEN_REGEX).map(t => (i += t.length))
}

export default TextContainer
