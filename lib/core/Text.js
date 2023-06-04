// TODO: https://github.com/cztomsik/graffiti/blob/bd1dfe61d3d7b5bfbf9184ecfb9e068dda982a60/src/dom/Text.ts

import { CharacterData } from './CharacterData.js'

export class Text extends CharacterData {
  get nodeType() {
    return Node.TEXT_NODE
  }

  get nodeName() {
    return '#text'
  }
}
