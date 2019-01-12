---
nav_order: 30
---

# React bindings
You don't need typescript but it's probably the most comfortable way to start.

```bash
mkdir hello-app
cd hello-app
npm init -y
npm i node-webrender react@next react-reconciler@next yoga-layout
npm i ts-node typescript --save-dev
```

Then you can create `main.tsx` with:

```js
import * as React from 'react'
import { Window } from 'node-webrender'
import { render, View, Text } from 'node-webrender/lib/react'

const App = () =>
  <View>
    <Text>Hello</Text>
  </View>

render(<App />, new Window("Hello"))
```

and it should show some text if you run it with `npx ts-node -O '{"jsx": "react"}' main.tsx`.


## Notes

- for `react-devtools` you need to add `react-devtools-core` and `ws` to your project
- you need to wrap styles in StyleSheet.create() if you want to use auto-completion (and better performance)
