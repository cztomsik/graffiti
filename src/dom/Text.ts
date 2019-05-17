export class Text {
  _data
  parentNode?

  constructor(data) {
    this._data = data
  }

  get data() {
    return this._data
  }

  set data(text) {
    this._data = text

    if (this.parentNode) {
      this.parentNode.updateText()
    }
  }
}
