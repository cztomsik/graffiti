import { HTMLElement } from './HTMLElement'

export class HTMLInputElement extends HTMLElement {
  tabIndex = 0
  type = 'text'
  _value = ''
  _textEl = this.ownerDocument.createElement('span')

  //name = ''
  //defaultValue = ''
  //checked = false
  //defaultChecked = false

  // preact sniffing
  onchange = null

  _created() {
    this.appendChild(this._textEl)

    this.addEventListener('keypress', e => {
      this.value += e.key

      this._fire('input')
      this._fire('change')

      console.log(this._listeners)
    })
  }

  get value() {
    return this._value
  }

  set value(v) {
    this._textEl.textContent = this._value = '' + v
  }
}
