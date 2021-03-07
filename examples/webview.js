import { App, WebView } from '../lib/index.js'

const app = await App.init()
const w = app.createWindow()

const HTML = `
  <style>
    body { font-family: sans-serif }
  </style>

  <h1>WebView</h1>

  <p>
    Useful for external sign-in and for showing web content. You can also
    start your own web-server and point it there if you want. Or pass HTML in
    data url like here.
  </p>

  <a href="https://excalidraw.com/">Start excalidraw</a>
`

// maybe it should be just new WebView() because that's definitely what people will try to do
const webview = app.createWebView()

webview.attach(w)
webview.loadURL(`data:text/html,${encodeURIComponent(HTML)}`)

// TODO: events

setTimeout(() => webview.eval('alert("Hello from WebView")'), 1000)

app.run()
