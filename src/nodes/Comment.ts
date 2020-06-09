import { Node } from './Node'
import { CharacterData } from './CharacterData'
import { Document } from './Document'

export class Comment extends CharacterData implements globalThis.Comment {
  constructor(public data = '', doc = document as Document) {
    super(doc)
  }

  get nodeType() {
    return Node.COMMENT_NODE
  }

  get nodeName() {
    return '#comment'
  }
}
