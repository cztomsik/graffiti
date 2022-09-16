export class NodeList<T extends Node> extends Array implements globalThis.NodeList, NodeListOf<T> {
  item(index: number): T {
    return this[index]
  }

  // TODO
  forEach
}

declare global {
  interface NodeList {
    slice
    splice
    filter
    find
  }
}
