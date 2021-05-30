// TODO: current TS should know this
interface NodeListOf<TNode extends Node> extends NodeList {
  [Symbol.iterator](): Iterator<TNode>
}
