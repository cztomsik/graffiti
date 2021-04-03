import { Node, NodeList, Document } from './index'

export class DocumentFragment extends Node implements globalThis.DocumentFragment {
  readonly childNodes = new NodeList<ChildNode>()

  constructor(doc = document as Document) {
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
}
