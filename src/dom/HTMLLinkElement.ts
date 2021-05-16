import { readURL } from '../util'
import { HTMLElement } from './HTMLElement'

export class HTMLLinkElement extends HTMLElement implements globalThis.HTMLLinkElement {
  get rel() {
    return this.getAttribute('rel') ?? ''
  }

  set rel(rel) {
    this.setAttribute('rel', rel)
  }

  get href() {
    return this.getAttribute('href') ?? ''
  }

  set href(href) {
    this.setAttribute('href', href)
  }
  
  // later
  as
  crossOrigin
  disabled
  hreflang
  imageSizes
  imageSrcset
  integrity
  media
  referrerPolicy
  relList
  sizes
  type
  nonce?
  sheet

  // deprecated
  charset
  rev
  target
}

// for now, we replace <link> with <style> which works surprisingly well
export async function loadStyles() {
  for (const link of document.querySelectorAll('link')) {
    if (link.rel === 'stylesheet' && link.href) {
      const style = document.createElement('style')
      style.textContent = await readURL('' + new URL(link.href, document.URL))
      link.replaceWith(style)
    }
  }
}
