import * as React from 'react'
import { Window } from '..'
import { View, Button, Text, render } from '../src/react'

const App = () =>
  <View style={{flexDirection: 'row', backgroundColor: '#ff0000'}}>
    <View style={{flex: 1, backgroundColor: '#cccccc'}}>
      <Text>Hello</Text>
      <Text>World dddddd</Text>
    </View>
    <View style={{flex: 2, backgroundColor: '#666666'}}>
      <Text>Olalala</Text>
      <View style={{ width: 100, height: 10, backgroundColor: '#0000ff' }} />
      {/*<Button title="Hello" />}
    </View>
  </View>

const w = new Window("Example")
render(<App />, w)

// prevent GC
global.window = w

setInterval(() => console.log('tick'), 60 * 1000)
*/
