export class NodeList<T extends Node> extends Array implements globalThis.NodeList, NodeListOf<T> {
  item(index: number): T {
    return this[index]
  }

  forEach

  static EMPTY_FROZEN: NodeList<any> = Object.freeze(new NodeList()) as any
}

declare global {
  interface NodeList {
    slice
    splice
    filter
    find
  }
}
