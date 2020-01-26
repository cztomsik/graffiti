import * as React from 'react'
import { render } from 'react-dom'

const Calculator = () => {
  const [expr, setExpr] = React.useState('0')

  const CaButton = ({ ch }) => (
    <div style={styles.caButton}>
      <button onClick={() => setExpr(expr + ch)}>{ch}</button>
    </div>
  )

  return (
    <div style={styles.container}>
      <Display value={expr} />

      <div style={styles.buttons}>
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
      </div>
    </div>
  )
}

const Display = ({ value }) => (
  <div style={styles.display}>
    <span style={styles.displayText}>{value}</span>
  </div>
)

const styles = {
  container: {
    height: '100%',
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
}

render(<Calculator />, document.body)
