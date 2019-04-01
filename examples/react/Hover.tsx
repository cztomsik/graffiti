import * as React from 'react'
import { useState } from 'react'
import { View, Text } from '../../src/react'

const Hover = () => {
  const [hovered, hover] = useState(false)

  return <View style={[{ padding: 10, backgroundColor: '#cccccc' }, hovered && { backgroundColor: '#888888' }]} onMouseOver={() => hover(true)} onMouseOut={() => hover(false)}>
    <Text>Hover me</Text>
  </View>
}

export { Hover }
