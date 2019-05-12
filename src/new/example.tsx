// node -r ts-node/register -r ./global.ts example.tsx

import * as React from 'preact'
import { useState } from 'preact/hooks'

// React.render(<span>Hello</span>, document.body)

React.render(<Counter />, document.body)

function Counter() {
  const [count, setCount] = useState(0)
  const inc = () => setCount(count + 1)

  return (
    <div onClick={inc} style={{ padding: 20 }}>
      <div style={{ backgroundColor: '#ccc' }}>
        <span>Click me {count}</span>
      </div>
    </div>
  )
}
