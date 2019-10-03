const ref = require('ref')
const ffi = require('ffi')

const lib = ffi.Library(
  `${__dirname}/libgraffiti/target/debug/libgraffiti`,
  {
    init: ['void', []],
    // pass a buffer (pointer to some memory + its length)
    send: [
      'void',
      [ref.refType(ref.types.void), 'int', ref.refType(ref.types.void)]
    ]
  }
)

lib.init()

const send = (msg) => {
  // fill with spaces (because of JSON)
  const resBuf = Buffer.alloc(1024, 0x20)

  const buf = Buffer.from(JSON.stringify(msg))

  lib.send(buf, buf.length, resBuf)

  const res = JSON.parse(resBuf.toString('utf-8'))

  if (res.error) {
    throw new Error(res.error)
  }

  return res
}

// TODO: create window
send({
  update: {
    alloc: 1
  }
})

setInterval(() => {
  // check for events
  const { events } = send({
    window_id: 1
  })

  // TODO: handle/log events for each window

  console.log(events)
}, 1000)

setTimeout(() => {
  send({
    window_id: 1,

    update: {
      text_changes: [
        { surface: 0, text: { font_size: 50, line_height: 50, align: 'Left', text: 'Hello', color: { r: 255, g: 0, b: 0, a: 255 } } }
      ]
    }
  })
}, 500)
