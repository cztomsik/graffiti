import { Node } from './Node'

type IDocumentFragment = globalThis.DocumentFragment

export class DocumentFragment extends Node implements IDocumentFragment {
  constructor(doc) {
    super(doc, Node.DOCUMENT_FRAGMENT_NODE)
  }

  // maybe later
  getElementById
  querySelector
  querySelectorAll
}
