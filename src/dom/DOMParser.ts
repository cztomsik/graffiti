// @ts-expect-error
import { Parser } from 'htmlparser2'
import { Document } from './index'
import { ERR } from '../util'

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
  html = html.replace(/[\s\S]*?(<\w+[\s\S]*)/i, '$1')

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

  const parser = new Parser({
    onopentag: (tag, atts) => {
      let el = doc.createElement(tag)
      Object.entries(atts ?? {}).forEach(([att, v]) => el.setAttribute(att, v))
      stack[0].appendChild(el)
      stack.unshift(el)
    },
    onclosetag: _ => stack.shift(),
    ontext: cdata => stack[0].appendChild(doc.createTextNode(cdata)),
    oncomment: cdata => stack[0].appendChild(doc.createComment(cdata)),
  })

  parser.write(html)
  parser.end()

  return fr
}
