import * as React from 'react'

class ControlManager {
  focusedControl = null

  focus(control) {
    this.blur()

    control.focus()

    this.focusedControl = control
  }

  blur() {
    if (this.focusedControl) {
      this.focusedControl.blur()
    }
  }

  keyPress(e) {
    if (this.focusedControl) {
      this.focusedControl.keyPress(e)
    }
  }

  // TODO: copy/paste()

  // TODO: arrows, home/end, pageup/down, ...
}

export const ControlManagerContext = React.createContext<ControlManager>(undefined)

export default ControlManager
