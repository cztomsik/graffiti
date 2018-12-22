import * as React from 'react'
import { Window } from '../src'
import { render, View, Text, ScrollView } from '../src/react'

const App = () =>
  <ScrollView style={{ flex: 1 }}>
    <Text>Top</Text>

    <View style={{ height: 400, backgroundColor: '#cccccc' }} />
    <View style={{ height: 400, backgroundColor: '#ff0000' }} />
    <View style={{ height: 400, backgroundColor: '#cccccc' }} />

    <Text>Bottom</Text>
  </ScrollView>

render(<App />, new Window("Hello", 200, 300))
