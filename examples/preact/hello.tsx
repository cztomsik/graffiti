// node -r ts-node/register -r ./src/register.ts examples/preact/hello.tsx

import * as React from 'preact'

React.render(<h1>Hello</h1>, document.body)
