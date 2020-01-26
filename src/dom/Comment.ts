import { Node } from './Node'

export class Comment extends Node {
  constructor(doc, public data, _nativeId) {
    super(doc, Node.COMMENT_NODE, _nativeId)
  }
}
