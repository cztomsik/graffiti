import { Node } from './Node.js'

export class XMLSerializer {
  serializeToString(node) {
    switch (node.nodeType) {
      case Node.TEXT_NODE:
        return escape(node.data)

      case Node.ELEMENT_NODE: {
        const el = node
        const tag = el.localName
        const attrs = el
          .getAttributeNames()
          .map(att => ` ${att}="${escape(el.getAttribute(att))}"`)
          .join('')
        const childNodes = el.childNodes.map(n => this.serializeToString(n)).join('')

        return `<${tag}${attrs}>${childNodes}</${tag}>`
      }

      case Node.DOCUMENT_NODE:
        return this.serializeToString(node.documentElement)

      case Node.DOCUMENT_FRAGMENT_NODE:
        return node.childNodes.map(n => this.serializeToString(n)).join('')
    }

    return ''
  }
}

const escape = str => str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;')
