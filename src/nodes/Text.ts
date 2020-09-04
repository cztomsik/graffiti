import { Node } from './Node'
import { CharacterData } from './CharacterData'

export class Text extends CharacterData implements globalThis.Text {
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
