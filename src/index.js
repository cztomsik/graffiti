const native = require('../native')

// there is a bug somehow related to garbage collection
// if a Window goes out of scope, the whole process will crash
const __gcBug = []

class Window extends native.Window {
  constructor(title) {
    super(title)
    __gcBug.push(this)
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
