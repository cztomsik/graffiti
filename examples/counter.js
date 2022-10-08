// ported from https://github.com/cztomsik/graffiti/blob/936b6e4bb5a51e138910a9315ecb91332012afb0/README.md
// the goal is to make it work again
import '../graffiti.js'
import { html, useState, render } from 'htm/preact/standalone.mjs'

const Counter = () => {
  const [count, setCount] = useState(0)
  const dec = () => setCount(count - 1)
  const inc = () => setCount(count + 1)

  return html`
    <div style=${styles.counter}>
      <span>${count}</span>

      <div style=${{ ...styles.bar, width: count * 5 }} />

      <div style=${styles.buttons}>
        <button onClick=${dec}>--</button>
        <button onClick=${inc}>++</button>
      </div>
    </div>
  `
}

const styles = {
  counter: {
    flex: 1,
    padding: 20,
    justifyContent: 'space-between',
  },

  bar: {
    backgroundColor: '#ff0000',
    height: 20,
  },

  buttons: {
    flexDirection: 'row',
    justifyContent: 'space-between',
  },
}

render(html`<${Counter} />`, document.body)
