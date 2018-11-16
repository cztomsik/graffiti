import * as React from 'react'
import { Window } from '..'
import { View, Button, Text, render } from '../src/react'

const App = () =>
  <View style={{ flexDirection: 'row', padding: 5, backgroundColor: '#ff0000' }}>
    <View style={{ flex: 1, backgroundColor: '#0000ff' }}>
    </View>
    <View style={{ flex: 2, backgroundColor: '#ffffff' }}>
      <View style={{ width: 100, height: 20, borderWidth: 1, borderColor: '#000000' }}></View>
    </View>
  </View>


render(<App />, new Window("Example"))

setInterval(() => console.log('tick'), 60 * 1000)
