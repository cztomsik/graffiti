import { native, register, getNativeId } from './native'
import { AppWindow } from './index'

export class WebView {
  constructor() {
    register(this, native.WebView_new())
  }

  attach(window: AppWindow) {
    native.WebView_attach(getNativeId(this), getNativeId(window))
  }

  async loadURL(url: URL | string) {
    native.WebView_load_url(getNativeId(this), '' + url)
  }

  async eval(js) {
    const res = native.WebView_eval(getNativeId(this), js)

    if (res !== undefined) {
      return JSON.parse(res)
    }
  }
}
