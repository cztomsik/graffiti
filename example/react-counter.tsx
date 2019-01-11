import * as React from 'react'
import { useState } from 'react'
import { Window } from '../src'
import { render, View, Text, Button } from '../src/react'

const App = () => {
  const [count, setCount] = useState(0)
  const dec = () => setCount(count - 1)
  const inc = () => setCount(count + 1)

  return (
    <View style={{ flex: 1, padding: 20, justifyContent: 'space-between' }}>
      <Text>{count}</Text>

      <View style={{ backgroundColor: '#ff0000', height: 20, width: count }} />

      <View style={{ flexDirection: 'row', justifyContent: 'space-between' }}>
        <Button title="--" onPress={dec} />
        <Button title="++" onPress={inc} />
      </View>
    </View>
  )
}

render(<App />, new Window("Counter", 250, 150))
