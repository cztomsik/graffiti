import { Document } from './index'
import { ERR } from '../util'
import { HTMLParser } from '../htmlparser.js'

export class DOMParser implements globalThis.DOMParser {
  parseFromString(string: string, contentType: DOMParserSupportedType): Document {
    switch (contentType) {
      case 'application/xhtml+xml':
      case 'text/html':
        return parseIntoDocument(new Document(), string)
    }

    return ERR(`unsupported type ${contentType}`)
  }
}

export const parseIntoDocument = (document, html: string) => {
  // strip <!DOCTYPE & other garbage
  html = html.replace(/[\s\S]*?(<\w+[\s\S]*)/i, '$1').trim()

  const frag = parseFragment(document, html)

  // happy-case
  if (frag.childNodes.length === 1 && frag.childNodes[0].localName === 'html') {
    document.appendChild(frag)
  } else {
    document.appendChild(parseFragment(document, '<html><head><title></title></head><body></body></html>'))
    document.body.appendChild(frag)
  }

  return document
}

export const parseFragment = (doc, html) => {
  const fr = doc.createDocumentFragment()
  let stack = [fr]

  HTMLParser(html, {
    start: (tag, atts) => {
      let el = doc.createElement(tag)
      atts.forEach((att) => el.setAttribute(att.name, att.value))
      stack[0].appendChild(el)
      stack.unshift(el)
    },
    end: _ => stack.shift(),
    chars: cdata => stack[0].appendChild(doc.createTextNode(cdata)),
    comment: cdata => stack[0].appendChild(doc.createComment(cdata)),
  })

  return fr
}
