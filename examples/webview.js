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
`

// maybe it should be just new WebView() because that's definitely what people will try to do
const webview = app.createWebView()

webview.attach(w)
webview.loadURL(`data:text/html,${encodeURIComponent(HTML)}`)

// TODO: events

setTimeout(
  () =>
    webview.eval(
      `document.body.innerHTML += ${JSON.stringify('<a href="https://excalidraw.com/">Start excalidraw</a>')}`
    ),
  1000
)

app.run()
