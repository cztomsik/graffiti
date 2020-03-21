import { Node } from './Node'

export class Comment extends Node implements globalThis.Comment {
  nodeType = Node.COMMENT_NODE
  data: string
}
