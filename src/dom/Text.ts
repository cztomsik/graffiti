import { initTextNode } from './Document'
import { Node, CharacterData } from './index'

export class Text extends CharacterData implements globalThis.Text {
  constructor(data, doc) {
    super(data, doc)

    // this.data is already normalized
    initTextNode(doc, this, this.data)
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
