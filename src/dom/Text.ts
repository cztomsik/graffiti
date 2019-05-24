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

    if (this.parentNode) {
      //this.parentNode.updateText()
    }
  }
}
