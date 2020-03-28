import { Node } from './Node'

export class Comment extends Node implements globalThis.Comment {
  constructor(doc, public data: string) {
    super(doc, Node.COMMENT_NODE)
  }
}
