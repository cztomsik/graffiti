import * as React from 'react'
import { useState } from 'react'
import { View, Button, Text } from '../../src/react'

export function Counter() {
  const [count, setCount] = useState(0)

  return (
    <View>
      <Text>{count}</Text>
      <Button title="++" onPress={() => setCount(count + 1)} />
    </View>
  )
}
