import * as React from 'react'
import { useState } from 'react'
import { render } from 'react-dom'

// preact works too
// /* @jsx h */
// import { h, render } from 'preact'
// import { useState } from 'preact/hooks'
// import 'preact/compat'

const TemperatureConverter = () => {
  const [celsius, setCelsius] = useState(NaN)
  const [fahrenheit, setFahrenheit] = useState(NaN)

  const inputCelsius = v => {
    setCelsius(v)
    setFahrenheit(+((9 / 5) * v + 32).toFixed(1))
  }

  const inputFahrenheit = v => {
    setFahrenheit(v)
    setCelsius(+((v - 32) * (5 / 9)).toFixed(1))
  }

  return (
    <div style={{ flexDirection: 'row', flexWrap: 'wrap', alignItems: 'center', padding: 20 }}>
      <NumInput value={celsius} onChange={inputCelsius} />
      <span>Celsius =</span>

      <NumInput value={fahrenheit} onChange={inputFahrenheit} />
      <span>Fahrenheit</span>

      <div style={{ width: '100%', margin: 20 }}>
        Enter number into either of the above, the other value will be recomputed instantly
      </div>
    </div>
  )
}

const NumInput = ({ value, onChange, ...rest }) => {
  const [focused, setFocused] = useState(false)

  return (
    <input
      style={{ backgroundColor: focused ? '#eef' : '#eee', width: 60, margin: 10 }}
      value={numToStr(value)}
      onFocus={() => setFocused(true)}
      onBlur={() => setFocused(false)}
      onChange={e => onChange(+e.target['value'])}
      {...rest}
    />
  )
}

const numToStr = v => (Number.isNaN(v) ? '' : '' + v)

render(<TemperatureConverter />, document.body)
