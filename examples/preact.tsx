/* @jsx h */
import { h, render } from 'preact'
import { useState } from 'preact/hooks'

const App = () => {
  const [text, setText] = useState('-- Click me --')

  return (
    <div style={{ padding: 20 }}>
      <h1>Hello</h1>

      <button onClick={() => setText(text.slice(-1) + text.slice(0, -1))}>{text}</button>
    </div>
  )
}

render(<App />, document.body)
