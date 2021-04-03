import { initTextNode } from './Document'
import { Node, CharacterData } from './index'

export class Text extends CharacterData implements globalThis.Text {
  constructor(data, doc) {
    super(data, doc)
    initTextNode(doc, this, data)
  }

  get nodeType() {
    return Node.TEXT_NODE
  }

  get nodeName() {
    return '#text'
  }

  // TODO
  wholeText
  splitText
}
