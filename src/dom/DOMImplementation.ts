import { UNSUPPORTED } from '../util'
import { DOMParser } from './index'

export class DOMImplementation implements globalThis.DOMImplementation {
  createDocument(namespaceURI: string | null, qualifiedName: string | null, doctype: DocumentType | null): Document {
    return new DOMParser().parseFromString(`<${qualifiedName}>`, 'text/xml')
  }

  createDocumentType(qualifiedName: string, publicId: string, systemId: string): DocumentType {
    // return new DocumentType(?, qualifiedName, publicId, systemId)
    return UNSUPPORTED()
  }

  createHTMLDocument(title?: string): Document {
    const document = new DOMParser().parseFromString('', 'text/html')
    document.title = title ?? ''

    return document
  }

  hasFeature(...args: any[]): true {
    return true
  }
}
