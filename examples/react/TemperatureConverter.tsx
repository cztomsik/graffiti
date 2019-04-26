import * as React from 'react'
import { useState } from 'react'
import { View, Button, Text, TextInput } from '../../src/react'

export function TemperatureConverter() {
  const [celsius, setCelsius] = useState(NaN)
  const [fahrenheit, setFahrenheit] = useState(NaN)

  const inputCelsius = (value: string) => {
    setCelsius(+value)
    setFahrenheit(+((9 / 5) * +value + 32).toFixed(1))
  }

  const inputFahrenheit = (value: string) => {
    setFahrenheit(+value)
    setCelsius(+((+value - 32) * (5 / 9)).toFixed(1))
  }

  return (
    <View style={{ flexDirection: 'row', flexWrap: 'wrap', alignItems: 'center' }}>
      <NumInput value={celsius} onChangeText={inputCelsius} />
      <Text>Celsius =</Text>
      <NumInput value={fahrenheit} onChangeText={inputFahrenheit} />
      <Text>Fahrenheit</Text>

      <Text style={{ margin: 10 }}>
        Enter number into either of the above, the other value will be recomputed instantly
      </Text>
    </View>
  )
}

const NumInput = ({ value, ...props }) => (
  <TextInput
    style={{ width: 60, margin: 10 }}
    value={numToStr(value)}
    {...props}
  />
)

const numToStr = v => (Number.isNaN(v) ? '' : '' + v)
