// react doesn't work yet
//import * as React from 'react'
//import { useState } from 'react'
//import { render } from 'react-dom'

/* @jsx h */
import { h, render } from 'preact'
import { useState } from 'preact/hooks'

const TemperatureConverter = () => {
  const [celsius, setCelsius] = useState(NaN)
  const [fahrenheit, setFahrenheit] = useState(NaN)

  const inputCelsius = e => {
    const value = +e.target.value
    setCelsius(value)
    setFahrenheit(+((9 / 5) * value + 32).toFixed(1))
  }

  const inputFahrenheit = e => {
    const value = +e.target.value
    setFahrenheit(value)
    setCelsius(+((value - 32) * (5 / 9)).toFixed(1))
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

const NumInput = ({ value, ...props }) => <input style={{ width: 60, margin: 10 }} value={numToStr(value)} {...props} />

const numToStr = v => (Number.isNaN(v) ? '' : '' + v)

render(<TemperatureConverter />, document.body)
