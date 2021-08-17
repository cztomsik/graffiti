import { Node, CharacterData } from './index'
import { normalize } from './CharacterData'
import { native, getNativeId, register } from '../native'

export class Text extends CharacterData implements globalThis.Text {
  constructor(data = '', doc = document) {
    super(doc)

    register(this, native.Document_create_text_node(getNativeId(doc), normalize(data)))
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
