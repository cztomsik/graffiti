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

We use [webrender](https://github.com/servo/webrender) for the actual drawing, yet there is **no DOM**, so we can be faster and yet use (much) less memory. [Read more here](./docs/webrender.md)

---

## Documentation
Please refer to respective sub-page on the
[project website](http://tomsik.cz/stain)
