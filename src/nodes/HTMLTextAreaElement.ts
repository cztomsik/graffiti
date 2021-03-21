import { HTMLElement } from './index'

export class HTMLTextAreaElement extends HTMLElement implements globalThis.HTMLTextAreaElement {
  type = 'textarea'

  _input = this.ownerDocument.createElement('input') as unknown as HTMLInputElement

  constructor(doc, tagName) {
    super(doc, tagName)

    this.appendChild(this._input)

    this.addEventListener('keydown', e => {
      if (e.code === 'Enter') {
        this.value += '\r\n'
      }
    })
  }

  get value() {
    return this._input.value
  }

  set value(v) {
    this._input.value = v
  }

  autocomplete
  checkValidity
  cols
  defaultValue
  dirName
  disabled
  form
  labels
  maxLength
  minLength
  name
  placeholder
  readOnly
  reportValidity
  required
  rows
  select
  selectionDirection
  selectionEnd
  selectionStart
  setCustomValidity
  setRangeText
  setSelectionRange
  textLength
  validationMessage
  validity
  willValidate
  wrap
}
