import { Node } from './Node'

export class Text extends Node {
  _data

  constructor(doc, data, _nativeId) {
    super(doc, Node.TEXT_NODE, _nativeId)
    this.data = data
  }

  get data() {
    return this._data
  }

  set data(text) {
    this._data = text

    if (this.parentElement) {
      this._updateText()
    }
  }

  _updateText() {
    const { fontSize, lineHeight } = this.parentElement.style._textStyle

    // TODO: get text style from parentElement
    this.ownerDocument._scene.setText(this._nativeId, fontSize, lineHeight, 0, this._data)
  }

  set textContent(v) {
    this.data = v
  }

  set nodeValue(v) {
    this.data = v
  }
}
