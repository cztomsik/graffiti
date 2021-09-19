import { native, register, getNativeId } from './native'
import { AppWindow } from './index'
import { encode } from './util'

export class WebView {
  constructor() {
    register(this, native.gft_WebView_new())
  }

  attach(window: AppWindow) {
    native.gft_WebView_attach(getNativeId(this), getNativeId(window))
  }

  async loadURL(url: URL | string) {
    native.gft_WebView_load_url(getNativeId(this), encode('' + url))
  }

  async eval(js) {
    const res = native.gft_WebView_eval(getNativeId(this), encode(js))

    if (res !== undefined) {
      return JSON.parse(res)
    }
  }
}
