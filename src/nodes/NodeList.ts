export class NodeList<T extends Node> extends Array implements globalThis.NodeList, NodeListOf<T> {
  item(index: number): T {
    return this[index]
  }

  forEach

  static empty(): NodeList<any> {
    return new NodeList()
  }
}

declare global {
  interface NodeList {
    splice
  }
}
