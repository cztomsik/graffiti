import { Node } from './Node'

export abstract class CharacterData extends Node implements globalThis.CharacterData {
  abstract data: string

  get length(): number {
    return this.data.length
  }

  // TODO
  appendData
  deleteData
  insertData
  replaceData
  substringData
}
