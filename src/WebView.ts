import { AppWindow } from './index'

export class WebView {
  #nativeApi: any
  #id: number

  constructor(nativeApi, id) {
    this.#nativeApi = nativeApi
    this.#id = id
  }

  attach(window: AppWindow) {
    // TODO: not sure about id visibility
    this.#nativeApi.webview_attach(this.#id, window.id)
  }

  async loadURL(url: URL | string) {
    this.#nativeApi.webview_load_url(this.#id, '' + url)
  }

  async eval(js) {
    const res = this.#nativeApi.webview_eval(this.#id, js)

    if (res !== undefined) {
      return JSON.parse(res)
    }
  }
}
