import { initComment } from './Document'
import { Node, CharacterData } from './index'

export class Comment extends CharacterData implements globalThis.Comment {
  constructor(data, doc) {
    super(data, doc)
    initComment(doc, this, data)
  }

  get nodeType() {
    return Node.COMMENT_NODE
  }

  get nodeName() {
    return '#comment'
  }
}
