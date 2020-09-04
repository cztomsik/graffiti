import { Node } from './Node'
import { CharacterData } from './CharacterData'

export class Comment extends CharacterData implements globalThis.Comment {
  get nodeType() {
    return Node.COMMENT_NODE
  }

  get nodeName() {
    return '#comment'
  }
}
