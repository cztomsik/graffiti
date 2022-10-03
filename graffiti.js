const mod = { exports: {} }
process.dlopen(mod, new URL('zig-out/lib/libgraffiti.dylib', import.meta.url).pathname)
console.log(mod.exports)
const native = mod.exports

class Node {
  get parentNode() {
    return native.Node_parentNode(this)
  }

  get firstChild() {
    return native.Node_firstChild(this)
  }

  get previousSibling() {
    return native.Node_previousSibling(this)
  }

  get nextSibling() {
    return native.Node_nextSibling(this)
  }

  appendChild(child) {
    native.Node_appendChild(this, child)
  }
}

class Element extends Node {
  setAttribute() {}
}

class Text extends Node {}

class Document extends Node {
  constructor() {
    return wrap(native.Document_init(), Document)
  }

  createElement(localName) {
    return wrap(native.Document_createElement(this, localName), Element)
  }

  createTextNode(data) {
    return wrap(native.Document_createTextNode(this, data), Text)
  }
}

const wrap = (obj, Clz) => (Object.setPrototypeOf(obj, Clz.prototype), obj)

global.document = new Document()
document.body = document.createElement('body')

setInterval(() => native.render(), 33)
