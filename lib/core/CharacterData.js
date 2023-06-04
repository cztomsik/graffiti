// TODO: https://github.com/cztomsik/graffiti/blob/bd1dfe61d3d7b5bfbf9184ecfb9e068dda982a60/src/dom/CharacterData.ts

import { native } from './native.js'
import { Node } from './Node.js'

export class CharacterData extends Node {
  get data() {
    return native.CharacterData_data(this)
  }

  set data(data) {
    native.CharacterData_setData(this, '' + data)
  }
}
