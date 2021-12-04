import { Node, CharacterData } from './index'
import { normalize } from './CharacterData'
import { native, encode, decode, getNativeId, register } from '../native'

export class Text extends CharacterData implements globalThis.Text {
  constructor(data = '', doc = document) {
    super(doc)

    register(this, native.gft_Document_create_text_node(getNativeId(doc), ...encode(normalize(data))))
  }

  get data() {
    return decode(native.gft_Text_data(getNativeId(this))) ?? ''
  }

  set data(data) {
    native.gft_Text_set_data(getNativeId(this), ...encode(normalize(data)))
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
