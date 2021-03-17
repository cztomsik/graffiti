// npm run build && npm run prepare && RUST_BACKTRACE=1 node examples/webview.js
// npm run build && npm run prepare && RUST_BACKTRACE=1 deno run -A --unstable examples/webview.js
import { App, AppWindow, WebView } from '../lib/index.js'

const app = await App.init()
const w = new AppWindow()

const HTML = `
  <style>
    body { font-family: sans-serif }
  </style>

  <h1>WebView <span id="time"></span></h1>

  <p>
    Useful for external sign-in and for showing web content. You can also
    start your own web-server and point it there if you want. Or pass HTML in
    data url like here.
  </p>

  <a href="https://excalidraw.com/">Excalidraw</a>
  <a href="https://vole.wtf/kilobytes-gambit/">Kilobyte's gambit</a>
`

const webview = new WebView()

webview.attach(w)
webview.loadURL(`data:text/html,${encodeURIComponent(HTML)}`)

// TODO: events

setInterval(
  () =>
    webview.eval(
      `document.querySelector('#time').innerHTML = ${JSON.stringify(new Date().toLocaleTimeString())}`
    ),
  1000
)

app.run()
