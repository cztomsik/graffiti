import * as React from 'react'
import { Window } from '..'
import { View, Button, Text, render } from '../src/react'

const App = () =>
  <View style={{ flexDirection: 'row' }}>
    <View style={{ flex: 1, padding: 10, backgroundColor: '#dddddd' }}>
      <Text>SIDEBAR</Text>
    </View>
    <View style={{ flex: 2, padding: 10 }}>
      <Button title="HELLO" />
    </View>
  </View>

render(<App />, new Window("Example"))
