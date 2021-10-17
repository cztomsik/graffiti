import { ERR } from '../util'
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

  querySelector(sel) {
    ERR('TODO: frag.querySelector() needs native fragment')
    return super.querySelector(sel)
  }

  querySelectorAll(sel) {
    ERR('TODO: frag.querySelectorAll() needs native fragment')
    return super.querySelectorAll(sel)
  }
}
