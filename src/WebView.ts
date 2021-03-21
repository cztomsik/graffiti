import { native } from './native'
import { AppWindow } from './index'
import { ID } from './AppWindow'

export class WebView {
  #id: number

  constructor() {
    this.#id = native.webview_new()

    WEBVIEW_REGISTRY.register(this, this.#id)
  }

  attach(window: AppWindow) {
    native.webview_attach(this.#id, window[ID])
  }

  async loadURL(url: URL | string) {
    native.webview_load_url(this.#id, '' + url)
  }

  async eval(js) {
    const res = native.webview_eval(this.#id, js)

    if (res !== undefined) {
      return JSON.parse(res)
    }
  }
}

const WEBVIEW_REGISTRY = new FinalizationRegistry(id => native.webview_free(id))
