import { HTMLElement } from './HTMLElement'
import { Event, InputEvent } from '../events/index'

// note that in react, `onChange` happens during typing too but
// in preact it does not (unless you also import `preact/compat`)
export class HTMLInputElement extends HTMLElement implements globalThis.HTMLInputElement {
  tabIndex = 0
  type = 'text'
  disabled = false
  readOnly = false
  name = ''
  defaultValue = ''
  checked = false
  defaultChecked = false
  accept = ''

  _value = ''
  _textNode: Text = this.ownerDocument.createTextNode('')

  constructor(doc, tagName) {
    super(doc, tagName)

    this.appendChild(this._textNode)

    this.addEventListener('keydown', e => {
      if (this.disabled || this.readOnly) {
        return
      }

      if (e.code === 'Backspace') {
        // see below
        this._value = this._value.slice(0, -1)
        this._updateText()

        this.dispatchEvent(
          new InputEvent('input', { bubbles: true, cancelable: true, data: e.key, inputType: 'deleteContentBackward' })
        )
      }
    })

    this.addEventListener('keypress', e => {
      if (this.disabled || this.readOnly) {
        return
      }

      // we can't just set this.value during keypress because
      // react installs its own getter/setter "tracker" (trackValueOnNode)
      // and then it would think no change event is needed (synthetic events)
      // so we need to set _value & then re-render
      this._value += e.key
      this._updateText()

      this.dispatchEvent(
        new InputEvent('input', { bubbles: true, cancelable: true, data: e.key, inputType: 'insertText' })
      )
    })

    // scope-private
    let prevValue = ''

    this.addEventListener('focus', () => (prevValue = this.value))

    this.addEventListener('blur', e => {
      if (prevValue !== this.value) {
        this.dispatchEvent(new Event('change'))
      }
    })
  }

  get value(): string {
    return this._value
  }

  set value(v) {
    const old = this._value

    this._value = '' + v

    if (this._value !== old) {
      // TODO: reset cursor (selectionStart, etc.)
    }

    this._updateText()
  }

  _updateText() {
    this._textNode.data = this._value
  }

  align
  alt
  autocomplete
  checkValidity
  dirName
  files
  form
  formAction
  formEnctype
  formMethod
  formNoValidate
  formTarget
  height
  indeterminate
  labels
  list
  max
  maxLength
  min
  minLength
  multiple
  pattern
  placeholder
  reportValidity
  required
  select
  selectionDirection
  selectionEnd
  selectionStart
  setCustomValidity
  setRangeText
  setSelectionRange
  size
  src
  step
  stepDown
  stepUp
  useMap
  validationMessage
  validity
  valueAsDate
  valueAsNumber
  width
  willValidate
}
