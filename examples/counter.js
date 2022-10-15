// originally restored from https://github.com/cztomsik/graffiti/blob/936b6e4bb5a51e138910a9315ecb91332012afb0/README.md
// the goal is to make it work again
import '../graffiti.js'
import { html, useState, render } from 'htm/preact/standalone.mjs'

const Counter = () => {
  const [count, setCount] = useState(10)
  const dec = () => setCount(count - 1)
  const inc = () => setCount(count + 1)

  return html`
    <div style=${styles.counter} onClick="{inc}">
      <div style="background-color: #f00">${count}</div>

      <div style="height: 20px"></div>

      <div style="background-color: #88f">${count}</div>

      <div style="height: 20px"></div>

      <div>
        <div style="background-color: #88f">world</div>
        <div style="background-color: #fff; flex-grow: 1">hello</div>
        <div style="background-color: #88f; width: 20px; height: 10px"></div>
      </div>

      <div style="height: 20px"></div>

      <div style="outline: 1 solid #000">outline</div>

      <div style="height: 20px"></div>

      <div style="box-shadow: 1 1 15 0 #004">shadow</div>
    </div>
  `
}

const styles = {
  counter: {
    // flex: 1,
    width: '80%',
    height: '100%',
    paddingTop: 20,
    paddingRight: 20,
    paddingBottom: 20,
    paddingLeft: 20,
    flexDirection: 'column',
    backgroundColor: '#ffa',
  },
}

render(html`<${Counter} />`, document.body)
