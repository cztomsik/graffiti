import { readURL } from '../util'
import { HTMLElement } from './index'

export class HTMLScriptElement extends HTMLElement implements globalThis.HTMLScriptElement {
  get type() {
    return this.getAttribute('type') ?? ''
  }

  set type(type) {
    this.setAttribute('type', type)
  }

  get src() {
    return this.getAttribute('src') ?? ''
  }

  set src(src) {
    this.setAttribute('src', src)
  }

  get text() {
    return this.textContent ?? ''
  }

  set text(text) {
    this.textContent = text
  }

  get async() {
    return this.hasAttribute('async')
  }

  set async(async) {
    this.toggleAttribute('async', async)
  }

  get defer() {
    return this.hasAttribute('defer')
  }

  set defer(defer) {
    this.toggleAttribute('defer', defer)
  }

  // later
  crossOrigin
  integrity
  noModule
  referrerPolicy
  nonce?

  // deprecated
  charset
  event
  htmlFor
}

export async function runScripts() {
  for (const script of document.querySelectorAll('script')) {
    // TODO: skip nomodule?

    if (script.type === 'module') {
      const url = script.src ? '' + new URL(script.src, document.URL) : createDataUrl(script.text)

      await import(url)
    } else {
      const url = script.src ? '' + new URL(script.src, document.URL) : document.URL

      evalScript(script.src ? await readURL(url) : script.text, url)
    }

    script.dispatchEvent(new Event('load'))
  }
}

function createDataUrl(script: string) {
  // prefix ./ and ../ with document.URL
  script = script.replace(/(import[^"']+["'])(\.\/)/g, (_, imprt) => `${imprt}${document.URL}/`)
  script = script.replace(/(import[^"']+["'])(\.\.\/)/g, (_, imprt) => `${imprt}${document.URL}/../`)

  return `data:text/javascript,base64${btoa(script)}`
}

// classic global-scope eval, vars & functions are accumulated
async function evalScript(script: string, filename: string) {
  return eval.call(null, `${script}\n//# sourceURL=${filename}`)
}
