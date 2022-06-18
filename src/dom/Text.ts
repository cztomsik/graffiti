import { IText } from '../types'
import { Node, CharacterData } from './index'
import { SEND, NODE_ID } from './Document'

export class Text extends CharacterData implements IText {
  constructor(data = '', doc = document) {
    super(data, doc)

    this[NODE_ID] = this.ownerDocument[SEND]({ CreateTextNode: this.data })
  }

  get data() {
    return super.data
  }

  set data(data) {
    super.data = data

    this.ownerDocument[SEND]({ SetText: [this[NODE_ID], this.data] })
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
