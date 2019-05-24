// node -r ts-node/register -r ./src/new/global.ts examples/preact.tsx

import * as React from 'preact'
import { useState } from 'preact/hooks'

// React.render(<span>Hello</span>, document.body)

React.render(<Counter />, document.body)

function Counter() {
  const [count, setCount] = useState(0)
  const inc = () => setCount(count + 1)

  return (
    <div onClick={inc} style={{ padding: 20 }}>
      <div style={{ content: 'Hello', backgroundColor: '#ccc', ...(count % 2 === 0 && { backgroundColor: '#f00' }) }}>
      </div>
    </div>
  )
}
