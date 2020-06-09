const { html, render, useState } = require('htm/preact/standalone.umd')

const App = () => {
  const [count, setCount] = useState(0)

  return html`
    <div>
      <h2>Count: ${count}</h2>

      <button onClick=${() => setCount(count + 1)}>++</button>
    </div>
  `
}

render(html`<${App} />`, document.body)
