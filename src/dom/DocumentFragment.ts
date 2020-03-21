import { Node } from './Node'

type IDocumentFragment = globalThis.DocumentFragment

export class DocumentFragment extends Node implements IDocumentFragment {
  nodeType = Node.DOCUMENT_FRAGMENT_NODE

  // maybe later
  getElementById
  querySelector
  querySelectorAll
}
