// node -r ts-node/register -r ./src/register.ts examples/react-dom/counter.tsx

import * as React from 'react'
import { render } from 'react-dom'

const Counter = () => {
  const [count, setCount] = React.useState(0)

  return (
    <div style={{ padding: 20 }}>
      <h1>Counter</h1>
      <span>{count}</span>
      <button onClick={() => setCount(count + 1)}>++</button>
    </div>
  )
}

render(<Counter />, document.body)
