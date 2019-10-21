import { Node } from "./Node";

export class Text extends Node {
  _data

  constructor(doc, data) {
    super(doc, Node.TEXT_NODE, -1)
    this._data = data
  }

  get data() {
    return this._data
  }

  set data(text) {
    this._data = text

    if (this.parentElement) {
      this.parentElement._setText(this._data)
    }
  }

  set textContent(v) {
    this.data = v
  }

  set nodeValue(v) {
    this.data = v
  }
}
