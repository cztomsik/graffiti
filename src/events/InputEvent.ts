import { UIEvent } from './index'

export class InputEvent extends UIEvent implements globalThis.InputEvent {
  data: string | null
  inputType: string
  isComposing: boolean

  constructor(type: string, eventInit?: InputEventInit) {
    super(type, eventInit)
    this.data = eventInit?.data ?? null
    this.inputType = eventInit?.inputType ?? 'insertText'
    this.isComposing = eventInit?.isComposing ?? false
  }
}
