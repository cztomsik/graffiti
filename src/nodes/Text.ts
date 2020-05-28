import { Node } from './Node'
import { CharacterData } from './CharacterData'

export class Text extends CharacterData implements globalThis.Text {
  _data: string

  constructor(data = '', doc: any = document) {
    super(doc)

    // preact passes data as is
    // (not sure if this is enough)
    if (typeof data === 'number') {
      data = '' + data
    }

    this._data = data

    this.ownerDocument._initTextNode(this, data)
  }

  get data() {
    return this._data
  }

  set data(data) {
    this._data = data

    // notify document
    this.ownerDocument._textUpdated(this, data)
  }

  get nodeType() {
    return Node.TEXT_NODE
  }

  get nodeName() {
    return '#text'
  }

  get nodeValue() {
    return this.data
  }

  set nodeValue(data: string) {
    this.data = data
  }

  get textContent() {
    return this.data
  }

  set textContent(data: string) {
    this.data = data
  }

  // TODO
  wholeText
  splitText
}
