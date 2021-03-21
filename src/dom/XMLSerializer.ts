// used for inner/outerHTML

import { Node } from '../nodes/Node'
import { Element } from '../nodes/Element'
import { Document } from '../nodes/Document'

// TODO: consider https://github.com/jsdom/w3c-xmlserializer
export class XMLSerializer implements globalThis.XMLSerializer {
  serializeToString(node: Node): string {
    switch (node.nodeType) {
      case Node.TEXT_NODE: {
        return node.nodeValue!.replace(/</g, '&lt;').replace(/>]/g, '&gt;')
      }

      case Node.ELEMENT_NODE: {
        const el = node as Element
        const tag = el.localName
        // TODO: escape
        const attrs = el
          .getAttributeNames()
          .map(att => `${att}="${el.getAttribute(att)}"`)
          .join(' ')
        const childNodes = el.childNodes.map(n => this.serializeToString(n)).join('')

        return `<${tag} ${attrs}>${childNodes}</${tag}>`
      }

      // TODO: Comment?

      // TODO: doctype?
      case Node.DOCUMENT_NODE:
        return this.serializeToString((node as Document).documentElement)

      case Node.DOCUMENT_FRAGMENT_NODE:
        return node.childNodes.map(n => this.serializeToString(n)).join('')
    }

    return ''
  }
}
