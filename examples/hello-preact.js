// node --experimental-network-imports examples/hello-preact
import '../graffiti.js'
// import { html, render } from 'https://unpkg.com/htm/preact/standalone.mjs'
// import { css } from 'https://unpkg.com/goober/dist/goober.esm.js'
import { html, render } from 'htm/preact/standalone.mjs'
import { css } from 'goober'

const clz = css`
  background: #f88;
`

render(html`<h1 className=${clz}>Hello</h1>`, document.body)
