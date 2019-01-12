---
title: Home
permalink: /
---

# node-webrender
[webrender](https://github.com/servo/webrender) bindings for node.js & react

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

Please note there is **no DOM, nor any web technology** involved in this project, unlike what you might expect from such name. [Read more here](./docs/webrender.md)

---

## Documentation
Please refer to respective sub-page on the
[project website](http://tomsik.cz/node-webrender)
