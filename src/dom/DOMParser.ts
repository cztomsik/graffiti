import { Document } from '../nodes/Document'
import { parseFragment } from '../nodes/Element'
import { ERR, TODO } from '../util'

export class DOMParser implements globalThis.DOMParser {
  parseFromString(string: string, contentType: DOMParserSupportedType): Document {
    switch (contentType) {
      case 'application/xml':
      case 'text/xml':
      case 'image/svg+xml':
        return TODO()

      case 'text/html': {
        // strip <!DOCTYPE & other garbage
        const html = string.replace(/[\s\S]*?(<\w+[\s\S]*)/i, '$1')

        // TODO: defaultView, URL
        const document = new Document()
        const frag = parseFragment(document, html)

        // happy-case
        if ((frag.childNodes.length === 1) && (frag.childNodes[0].localName === 'html')) {
          document.appendChild(frag)
          return document
        } else {
          return this.parseFromString(`<html><head><title></title></head><body>${html}</body></html>`, 'text/html')
        }
      }
    }

    return ERR(`invalid type ${contentType}`)
  }
}
