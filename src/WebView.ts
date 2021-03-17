import { native } from './native'
import { AppWindow } from './index'

export class WebView {
  _id: number

  constructor() {
    this._id = native.webview_new()

    REGISTRY.register(this, this._id)
  }

  attach(window: AppWindow) {
    native.webview_attach(this._id, window._id)
  }

  async loadURL(url: URL | string) {
    native.webview_load_url(this._id, '' + url)
  }

  async eval(js) {
    const res = native.webview_eval(this._id, js)

    if (res !== undefined) {
      return JSON.parse(res)
    }
  }
}

const REGISTRY = new FinalizationRegistry(id => native.webview_free(id))
