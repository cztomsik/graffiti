// node -r ts-node/register -r ./src/register.ts examples/react-dom/hello.tsx

import * as React from 'react'
import { render } from 'react-dom'

render(<h1>Hello</h1>, document.body)
