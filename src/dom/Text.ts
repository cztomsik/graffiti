import { IText } from '../types'
import { Node, CharacterData } from './index'
import { encode, native } from '../native'
import { DOC_ID, NODE_ID } from './Document'

export class Text extends CharacterData implements IText {
  constructor(data = '', doc = document) {
    super(data, doc)

    this[NODE_ID] = native.gft_Document_create_text_node(doc[DOC_ID], encode(data))
  }

  get data() {
    return super.data
  }

  set data(data) {
    super.data = data

    native.Text_set_data(this[NODE_ID], this.data)
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
