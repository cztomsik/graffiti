import { Node, CharacterData } from './index'

export class Comment extends CharacterData implements globalThis.Comment {
  get nodeType() {
    return Node.COMMENT_NODE
  }

  get nodeName() {
    return '#comment'
  }
}
