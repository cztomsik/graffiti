import 'graffiti'
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
    padding: 20,
    background: '#eee',
  },

  bar: {
    background: 'hsla(0turn, 100%, 50%, 1)',
    height: 20,
    margin: '10px 0',
  },

  buttons: {
    display: 'flex',
    justifyContent: 'space-between',
  },
}

render(html`<${Counter} />`, document.body)
