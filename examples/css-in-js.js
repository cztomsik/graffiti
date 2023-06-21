// node --experimental-network-imports examples/hello-preact-goober
import 'graffiti'
import { html, render } from 'https://unpkg.com/htm/preact/standalone.mjs'
import { css } from 'https://unpkg.com/goober/dist/goober.esm.js'

const wrapper = css`
  padding: 20px;

  h1 {
    background: #ff0;
    margin: 0;
  }

  div {
    background: #f00;
  }
`

const App = () => html`
  <div className=${wrapper}>
    <h1>CSS-in-JS</h1>
    <div>Preact + Goober</div>
  </div>
`

render(html`<${App} />`, document.body)
