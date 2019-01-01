import * as React from 'react'
import { useState } from 'react'
import { Window } from '../src'
import { render, View, TextInput, Button, StyleSheet } from '../src/react'

const App = () => {
  const [firstName, setFirstName] = useState('John')
  const [lastName, setLastName] = useState('Doe')
  const swap = () => (setLastName(firstName), setFirstName(lastName))

  return (
    <View style={styles.container}>
      <TextInput value={firstName} onChangeText={setFirstName} />

      <TextInput value={lastName} onChangeText={setLastName} />

      <Button title="Click me" onPress={swap} />
    </View>
  )
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    padding: 20,
    justifyContent: 'space-between'
  }
})

render(<App />, new Window('Hello', 300, 200))
