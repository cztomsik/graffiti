import * as React from 'react'
import { useState } from 'react'
import { View, Image } from '../../src/react'

export function ImageExample() {
  const [count, setCount] = useState(0)

  return (
    <View style={{ flexDirection: 'row' }}>
      <Image
        style={{ width: 200, height: 200 }}
        source={{ uri: 'docs/images/react-calculator.png' }}
      />
    </View>
  )
}
