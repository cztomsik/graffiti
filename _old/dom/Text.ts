import { IText } from '../types'
import { Node, CharacterData } from './index'
import { initText, setText } from './Document'

export class Text extends CharacterData implements IText {
  constructor(data = '', doc = document) {
    super(data, doc)

    doc[initText](this, this.data)
  }

  get data() {
    return super.data
  }

  set data(data) {
    super.data = data

    this.ownerDocument[setText](this, this.data)
  }

  get nodeType() {
    return Node.TEXT_NODE
  }

  get nodeName() {
    return '#text'
  }

  // TODO
  wholeText
  splitText
}
