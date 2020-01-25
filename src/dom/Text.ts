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

    // TODO: get text style from parentElement
    this.ownerDocument._scene.setText(this._nativeId, 16, 20, 0, this._data)
  }

  set textContent(v) {
    this.data = v
  }

  set nodeValue(v) {
    this.data = v
  }
}
