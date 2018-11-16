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
        left: { color: BLUE, style: 'Solid' },
        right: { color: BLUE, style: 'Solid' },
        top: { color: BLUE, style: 'Solid' },
        bottom: { color: BLUE, style: 'Solid'},
        radius: {
          top_left: [0, 0],
          top_right: [0, 0],
          bottom_left: [0, 0],
          bottom_right: [0, 0]
        },
        do_aa: false
      }
    }
  }
})

let i = 0

setInterval(() => {
  w.updateBucket(b1, {
    Rectangle: {
      color: (i % 5) > 1 ?RED :BLUE
    }
  })

  w.render({
    bucket_ids: [b1, b2],
    layouts: [
      // x, y, w, h
      [0, 0, 100, 100],
      [100, 100, 100, 50 * (1 + Math.sin(i))]
    ]
  })

  i += 0.1
}, 20)
