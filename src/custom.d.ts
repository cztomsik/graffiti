// TODO: update TS version & add esnext.weakref lib

declare class FinalizationRegistry {
  constructor(f: (heldValue: any) => {})
  register(target: object, heldValue: any)
}

declare class WeakRef<T> {
  constructor(v: T)
  deref(): T | undefined
}

// TODO: current TS should know this
interface NodeListOf<TNode extends Node> extends NodeList {
  [Symbol.iterator](): Iterator<TNode>
}

interface Object {
  fromEntries
}
