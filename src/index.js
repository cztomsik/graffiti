const native = require('../native')

// see https://github.com/cztomsik/node-webrender/issues/2
const __gcBug = []

class Window extends native.Window {
  constructor(title) {
    super(title)
    __gcBug.push(this)

    // keep the process up
    setInterval(() => {}, Math.pow(2, 17))
  }

  createBucket(item) {
    return super.createBucket(JSON.stringify(item))
  }

  updateBucket(bucket, item) {
    super.updateBucket(bucket, JSON.stringify(item))
  }

  render(request) {
    super.render(JSON.stringify(request))
  }
}

exports.Window = Window
