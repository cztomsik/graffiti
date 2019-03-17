---
title: Home
permalink: /
---

# stain
Rapid GUI development using familiar technologies (javascript, flexbox, css)

---

<div style="display: flex; align-items: center">
<div style="max-height: 400px; overflow-y: scroll">

```javascript
const App = () => {
  const [count, setCount] = useState(0)
  const dec = () => setCount(count - 1)
  const inc = () => setCount(count + 1)

  return (
    <View style={styles.counter}>
      <Text>{count}</Text>

      <View style={[styles.bar, { width: count * 5 }]} />

      <View style={styles.buttons}>
        <Button title="--" onPress={dec} />
        <Button title="++" onPress={inc} />
      </View>
    </View>
  )
}

const styles = StyleSheet.create({
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
})
```

</div>
<img src="./docs/images/counter.gif" />
</div>
<br>

## Why it's interesting
- quick to setup, apart from rust & few libs, it should be just one `npm install` away
- can be combined with most of the libraries you already know (react, mobx, lodash, ...)
- works with existing tooling (debug in vscode, profile in chrome devtools, react-devtools, ...)
- hot-reload works even without webpack (and it's faster)
- bundle can be made using already established and mature tools (ncc + electron-builder)
- low memory footprint (when compared to electron)
- the language & platform you already know (when compared to flutter)
- intention to support most of the CSS properties (when compared to react-native)

We use [webrender](https://github.com/servo/webrender) for the actual drawing, yet there is **no DOM**, so we can be faster and use (much) less memory. [Read more here](./docs/webrender.md)

---

## Documentation
Please refer to respective sub-page on the
[project website](http://tomsik.cz/stain)
