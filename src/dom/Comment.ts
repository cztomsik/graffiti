import { Node, CharacterData } from './index'
import { normalize } from './CharacterData'
import { native, encode, getNativeId, register } from '../native'

export class Comment extends CharacterData implements globalThis.Comment {
  constructor(public data = '', doc = document) {
    super(doc)
  }

  get nodeType() {
    return Node.COMMENT_NODE
  }

  get nodeName() {
    return '#comment'
  }
}
