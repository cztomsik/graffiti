import * as React from 'react'
import { useState } from 'react'
import { View, Button, Text, TextInput } from '../../src/react'

export function Hello() {
  const [value, setValue] = useState('test')

  return <View>
    <TextInput value={value} onChangeText={setValue} style={{ marginBottom: 20 }} />
    <Text>Hello {value}</Text>
  </View>
}
