import { Node, CharacterData } from './index'
import { normalize } from './CharacterData'
import { native, getNativeId, register } from '../native'
import { encode } from '../util'

export class Comment extends CharacterData implements globalThis.Comment {
  constructor(data = '', doc = document) {
    super(doc)

    register(this, native.gft_Document_create_comment(getNativeId(doc), encode(normalize(data))))
  }

  get nodeType() {
    return Node.COMMENT_NODE
  }

  get nodeName() {
    return '#comment'
  }
}
