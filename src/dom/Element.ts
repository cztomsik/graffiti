import { Node } from "./Node";
import { camelCase } from "../core/utils";

export class Element extends Node {
  id?
  _style = new CSSStyleDeclaration()

  constructor(public ownerDocument, public tagName, public _nativeId) {
    super(ownerDocument, Node.ELEMENT_NODE, _nativeId)
  }

  // bubble events
  // @see EventTarget
  _getTheParent() {
    return this.parentElement
  }

  get style() {
    return new Proxy(this._style, {
      set: (target, property, value) => {
        requestAnimationFrame(() => {
          const { flex, padding, backgroundColor } = compileFlatStyle(this._style as any)

          this.ownerDocument._scene.setFlex(this._nativeId, flex)
          this.ownerDocument._scene.setPadding(this._nativeId, padding)
          this.ownerDocument._scene.setBackgroundColor(this._nativeId, backgroundColor)
        })

        return Reflect.set(target, property, value)
      }
    })
  }

  setAttribute(name, value) {
    this[camelCase(name)] = value
  }

  removeAttribute(name) {
    delete this[camelCase(name)]
  }

  getBoundingClientRect() {
    return { x: 0, left: 0, y: 0, top: 0, width: 100, height: 100 }
  }

  get offsetWidth() {
    return 0
  }

  get offsetHeight() {
    return 0
  }
}
