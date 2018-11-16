import { Window } from '..'

const w = new Window("Hello")

const bucket = w.createBucket({
  Rectangle: {
    // rgba
    color: [1, 0, 0, 1]
  }
})

w.render({
  bucket_ids: [0],
  layouts: [
    // x, y, w, h
    [0, 0, 100, 100]
  ]
})

// stay alive & prevent GC
setInterval(() => console.log(w), 60 * 1000)
