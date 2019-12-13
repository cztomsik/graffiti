---
title: Home
permalink: /
---

# graffiti
Experimental GUI toolkit for node.js. The idea is to implement just enough subset of DOM & styling so that you can use react & other web frameworks, including the most of the related libraries but keep it as simple as possible so it can run fluently even on cheap devices like Raspberry Pi. The trick is to render everything using GPU and to keep the scope & abstractions at minimum.

One day, it could be viable alternative to [electron](https://github.com/electron/electron) but you will never be able to just take an existing web app and run it with this (but it should be possible the other way around).

---

<div style="display: flex; align-items: center">
<div style="max-height: 400px; overflow-y: scroll">

```javascript
import * as React from 'react'
import { render } from 'react-dom'

const Counter = () => {
  const [count, setCount] = React.useState(0)
  const dec = () => setCount(count - 1)
  const inc = () => setCount(count + 1)

  return (
    <div style={styles.counter}>
      <span>{count}</span>

      <div style={{ ...styles.bar, width: count * 5 }} />

      <div style={styles.buttons}>
        <button onClick={dec}>--</button>
        <button onClick={inc}>++</button>
      </div>
    </div>
  )
}

const styles = {
  counter: {
    flex: 1,
    padding: 20,
    justifyContent: 'space-between'
  },

  bar: {
    backgroundColor: '#ff0000',
    height: 20
  },

  buttons: {
    flexDirection: 'row',
    justifyContent: 'space-between'
  }
}

render(<Counter />, document.body)
```

</div>
<img src="./docs/images/counter.gif" />
</div>
<br>

## Status
Please note that we are still finishing major rewrite which started almost 3 months ago (yeah, I know but it was really needed) so the docs are off (a lot). That said, we've also gained some other cool features along the way (text-align, images, shadow). As with other hobby projects, there are no ETAs, but it's very close now. Follow me on my [twitter](https://twitter.com/cztomsik) to get notified :-)

## Why it's interesting
- quick to setup, apart from rust, it should be just one `npm install` away
- can be combined with most of the libraries you already know (react, mobx, lodash, ...)
- works with existing tooling (debug in vscode, profile in chrome devtools, react-devtools, ...)
- hot-reload works even without webpack (and it's faster)
- bundle can be made using already established and mature tools (ncc + electron-builder)
- low memory footprint (when compared to electron)
- the language & platform you already know (when compared to flutter)

---

## Documentation
Please refer to respective sub-page on the
[project website](http://tomsik.cz/graffiti)

## License
This project is [MIT licensed](./LICENSE). By contributing to this project, you also agree that your contributions will be licensed under the MIT license.
