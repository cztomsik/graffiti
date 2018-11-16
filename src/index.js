const native = require('../native')

class Window extends native.Window {
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
