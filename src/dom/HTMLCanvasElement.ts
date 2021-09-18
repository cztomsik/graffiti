// TODO

import { HTMLElement } from './HTMLElement'
//import { WebGLRenderingContext } from '../webgl/WebGLRenderingContext'
import { UNSUPPORTED } from '../util'

export class HTMLCanvasElement extends HTMLElement implements globalThis.HTMLCanvasElement {
  captureStream
  width
  height
  nonce

  getContext(contextId, options?) {
    // TODO: save it for next time,
    //       return null if not supported or different type requested than before

    if (contextId === '2d') {
      //return new CanvasRenderingContext2D(...)
    }

    if (contextId === 'webgl') {
      return new WebGLRenderingContext()
    }

    return UNSUPPORTED()
  }

  toBlob(callback: BlobCallback, type?: string, quality?: any): void {
    return UNSUPPORTED()
  }

  toDataURL(type?: string, quality?: any): string {
    return UNSUPPORTED()
  }
}
