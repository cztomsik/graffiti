export class Window {
  id = 0

  constructor(title, width = 800, height = 600) {
    // TODO: factory (no side-effects in constructor)

    // TODO (in create/openWindow)
    // it's not constructor's job to perform window allocation and so it shouldn't free either
    // createdWindow.destructor = require('finalize')(window, function() { send(destroyWindow(window.id)) }))
  }

  // maybe this should be done somewhere else - if this class doesn't do aquire, maybe it shouldn't do destroy either
  // and then maybe it's not
  // TODO: destructor = require('finalize')(this, () => send(destroyWindow(this.id)))


  setSize(width, height) {
    // TODO (sync)
  }

  close() {
    // TODO (sync)
  }
}

export const __callbacks = []
