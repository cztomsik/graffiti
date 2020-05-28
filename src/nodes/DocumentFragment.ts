import { Node } from './Node'
import { NodeList } from './NodeList'

export class DocumentFragment extends Node implements globalThis.DocumentFragment {
  childNodes = new NodeList<ChildNode>()

  constructor(doc = document) {
    super(doc)
  }

  get nodeType() {
    return Node.DOCUMENT_FRAGMENT_NODE
  }

  get nodeName() {
    return '#document-fragment'
  }

  // maybe later
  getElementById
  querySelector
  querySelectorAll
}
