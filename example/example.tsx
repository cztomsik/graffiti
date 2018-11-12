import * as React from 'react'
import { Window } from '..'
import { View, Button, render } from '../src/react'

const App = () =>
  <View style={{flexDirection: 'row', backgroundColor: '#ff0000'}}>
    <View style={{flex: 1, backgroundColor: '#cccccc'}} />
    <View style={{flex: 2, backgroundColor: '#666666'}}>
      {/*<Button title="Hello" />*/}
    </View>
  </View>

const w = new Window("Example")
render(<App />, w)

// prevent GC
global.window = w

setInterval(() => console.log('tick'), 10 * 1000)
