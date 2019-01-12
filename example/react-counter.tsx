import * as React from 'react'
import { useState } from 'react'
import { Window } from '../src'
import { render, View, Text, Button, StyleSheet } from '../src/react'

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

render(<App />, new Window("Counter", 200, 150))
