import * as React from 'react'
import { useState } from 'react'
import { Window } from '../src'
import { render, View, Text, Button, StyleSheet } from '../src/react'

const App = () => {
  const [expr, setExpr] = useState('0')

  const CaButton = ({ ch }) =>
    <View style={styles.caButton}>
      <Button title={ch} onPress={() => setExpr(expr + ch)} />
    </View>

  return (
    <View style={{ flex: 1 }}>
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

const Display = ({ value }) =>
  <View style={styles.display}>
    <Text style={styles.displayText}>{value}</Text>
  </View>

const styles = StyleSheet.create({
  display: {
    height: 100,
    padding: 10,
    justifyContent: 'flex-end',
    backgroundColor: '#000000'
  },

  displayText: {
    color: '#ffffff'
  },

  buttons: {
    flex: 1,
    flexDirection: 'row',
    flexWrap: 'wrap'
  },

  caButton: {
    width: '25%'
  }
})

render(<App />, new Window("Calculator", 400, 300))
