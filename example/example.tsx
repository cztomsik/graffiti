import * as React from 'react'
import { Window } from '..'
import { View, Button, Text, render } from '../src/react'

const App = () =>
  <View style={styles.container}>
    <View style={styles.sidebar}>
      <Text>Sidebar</Text>

      <Text>Lorem ipsum dolor sit amet</Text>
    </View>
    <View style={styles.main}>
      <Text>Main content</Text>

      <Button title="Click to exit" onPress={() => process.exit()} />
    </View>
  </View>

const styles = {
  container: {
    flexDirection: 'row'
  },

  sidebar: {
    flex: 1,
    padding: 20,
    backgroundColor: '#eeeeee',
    justifyContent: 'space-between'
  },

  main: {
    flex: 2,
    padding: 20
  }
}

render(<App />, new Window("Example"))
