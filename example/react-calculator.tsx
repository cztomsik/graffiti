import * as React from 'react'
import { useState } from 'react'
import { Window } from '../src'
import { render, View, Text, Button, StyleSheet } from '../src/react'

const App = () => {
  const [expr, setExpr] = useState('0')

  const CaButton = ({ ch }) => (
    <View style={styles.caButton}>
      <Button title={ch} onPress={() => setExpr(expr + ch)} />
    </View>
  )

  return (
    <View style={styles.container}>
      <Display value={expr} />

      <View style={styles.buttons}>
        <CaButton ch="7" />
        <CaButton ch="8" />
        <CaButton ch="9" />
        <CaButton ch="*" />

        <CaButton ch="4" />
        <CaButton ch="5" />
        <CaButton ch="6" />
        <CaButton ch="-" />

        <CaButton ch="1" />
        <CaButton ch="2" />
        <CaButton ch="3" />
        <CaButton ch="+" />

        <CaButton ch="0" />
        <CaButton ch="," />
        <CaButton ch="/" />
        <CaButton ch="=" />
      </View>
    </View>
  )
}

const Display = ({ value }) => (
  <View style={styles.display}>
    <Text style={styles.displayText}>{value}</Text>
  </View>
)

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#444466'
  },

  display: {
    height: 80,
    padding: 10,
    justifyContent: 'flex-end',
    backgroundColor: '#000000'
  },

  displayText: {
    fontSize: 20,
    color: '#ffffff'
  },

  buttons: {
    padding: 10,
    paddingHorizontal: 5,
    flex: 1,
    flexDirection: 'row',
    flexWrap: 'wrap'
  },

  caButton: {
    width: '25%',
    padding: 3
  }
})

render(<App />, new Window('Calculator', 230, 350))
