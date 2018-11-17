import { Window } from '..'

// rgba
const RED = [1, 0, 0, 1]
const BLUE = [0, 0, 1, 1]

const w = new Window("Hello")

const b1 = w.createBucket({
  Rectangle: {
    color: RED
  }
})

const b2 = w.createBucket({
  Border: {
    widths: [1, 1, 1, 1],
    details: {
      Normal: {
        top: { color: BLUE, style: 'Solid' },
        right: { color: BLUE, style: 'Solid' },
        bottom: { color: BLUE, style: 'Solid'},
        left: { color: BLUE, style: 'Solid' },
        radius: {
          top_left: [5, 5],
          top_right: [5, 5],
          bottom_left: [5, 5],
          bottom_right: [5, 5]
        },
        // has to be true for radius (or we get whitescreen)
        do_aa: true
      }
    }
  }
})

const [aGlyph] = w.getGlyphIndices("A")
const b3 = w.createBucket({
  Text: [
    { font_key: [1, 2], color: RED },
    [
      [aGlyph, [220, 130]]
    ]
  ]
})


let i = 0

setInterval(() => {
  w.updateBucket(b1, {
    Rectangle: {
      color: [1, Math.abs(Math.sin(i)), 0, 1]
    }
  })

  w.render({
    bucket_ids: [b1, b2, b3],
    layouts: [
      // x, y, w, h
      [0, 0, 100, 100],
      [100, 100, 100, 50 * (1 + Math.sin(i))],
      [200, 100, 100, 30]
    ]
  })

  i += 0.1
}, 20)
