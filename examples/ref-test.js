// npm run build && npm run prepare && deno run -Ar --unstable examples/ref-test.js

import { App, AppWindow, WebView } from '../lib/index.js'

const app = await App.init()

const w1 = new AppWindow()
// await w1.loadURL(new URL('hello.html', import.meta.url))
await w1.loadURL(new URL('calculator.html', import.meta.url))

// TODO: the trailing / is mandatory
// await w1.loadURL('https://perfectmotherfuckingwebsite.com/')
// await w1.loadURL('https://developit.github.io/hn_minimal/')
// await w1.loadURL('http://svelte-todomvc.surge.sh/')
// await w1.loadURL('https://developit.github.io/preact-todomvc/')
// await w1.loadURL('http://localhost:3000/')

const w2 = new AppWindow()
const webview = new WebView()
webview.attach(w2)

setInterval(async () => {
  const html = await w1.eval('document.documentElement.outerHTML')
  webview.loadURL(`data:text/html,${encodeURIComponent(html)}`)
}, 1000)

app.run()
