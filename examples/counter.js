// node --experimental-network-imports examples/counter.js
// originally restored from https://github.com/cztomsik/graffiti/blob/936b6e4bb5a51e138910a9315ecb91332012afb0/README.md
// the goal is to make it work again
import '../graffiti.js'
import { html, useState, render } from 'https://unpkg.com/htm/preact/standalone.mjs'

const Counter = () => {
  const [count, setCount] = useState(10)
  const dec = () => setCount(count - 1)
  const inc = () => setCount(count + 1)

  return html`
    <div style=${styles.counter}>
      <span>${count}</span>

      <div style=${{ ...styles.bar, width: count * 5 }} />

      <div style=${styles.buttons}>
        <button style=${styles.button} onClick=${dec}>--</button>
        <button style=${styles.button} onClick=${inc}>++</button>
      </div>
    </div>
  `
}

const styles = {
  counter: {
    width: 400,
    height: 300,
    backgroundColor: '#ff0',
    display: 'flex',
    flexDirection: 'column',
    padding: 20,
    justifyContent: 'space-between',
  },

  bar: {
    backgroundColor: '#f00',
    height: 20,
  },

  buttons: {
    justifyContent: 'space-between',
  },

  button: {
    backgroundColor: '#22f',
  },
}

render(html`<${Counter} />`, document.body)
