import { Node } from './Node'
import { CharacterData } from './CharacterData'

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

  get nodeValue() {
    return this.data
  }

  set nodeValue(data: string) {
    this.data
  }

  get textContent() {
    return this.data
  }

  set textContent(data: string) {
    this.data = data
  }
}
