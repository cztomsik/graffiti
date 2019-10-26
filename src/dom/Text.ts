import { Node } from "./Node";

export class Text extends Node {
  _data

  constructor(doc, data) {
    super(doc, Node.TEXT_NODE, undefined)
    this._data = data
  }

  get data() {
    return this._data
  }

  set data(text) {
    this._data = text

    if (this.parentElement) {
      this.parentElement._updateText()
    }
  }

  set textContent(v) {
    this.data = v
  }

  set nodeValue(v) {
    this.data = v
  }
}
