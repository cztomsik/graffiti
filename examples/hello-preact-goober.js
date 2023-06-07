// node --experimental-network-imports examples/hello-preact-goober
import 'graffiti'
import { html, render } from 'https://unpkg.com/htm/preact/standalone.mjs'
import { css } from 'https://unpkg.com/goober/dist/goober.esm.js'

const clz = css`
  background: #f88;
`

render(html`<h1 className=${clz}>Hello</h1>`, document.body)
