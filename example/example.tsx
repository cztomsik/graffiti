import * as React from 'react'
import { Window } from '../src'
import { View, Button, Text, StyleSheet, render, Image } from '../src/react'

import { readFileSync } from 'fs'

const source = readFileSync(__filename, 'utf-8')

const App = () => (
  <View style={styles.container}>
    <View style={[styles.box]}>
      <Text style={styles.heading}>stain</Text>
      <Text>Rapid GUI prototyping using familiar technologies</Text>

      <Button title="Click to exit" onPress={() => process.exit()} />

      <Image
        style={styles.img}
        source={{ uri: 'docs/images/react-calculator.png' }}
      />
    </View>

    <View style={[styles.box, styles.source]}>
      <Text style={styles.source}>{source}</Text>
    </View>
  </View>
)

const styles = StyleSheet.create({
  container: {
    flex: 1,
    flexDirection: 'row'
  },

  box: {
    flex: 1,
    padding: 20
  },

  heading: {
    fontSize: 24,
    lineHeight: 30
  },

  img: {
    margin: 50,
    width: 200,
    height: 200,
    shadowColor: '#888888',
    shadowRadius: 10
  },

  source: {
    fontSize: 12,
    lineHeight: 14,
    backgroundColor: '#272822',
    color: '#f8f8f2'
  }
})

render(<App />, new Window('Example'))
