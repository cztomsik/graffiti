import { Node } from './Node'
import { CharacterData } from './CharacterData'
import { NodeList } from './NodeList'

export class Comment extends CharacterData implements globalThis.Comment {
  constructor(public data = '', doc = document) {
    super(doc)
  }

  get childNodes() {
    return NodeList.empty()
  }

  get nodeType() {
    return Node.COMMENT_NODE
  }

  get nodeName() {
    return '#comment'
  }
}
