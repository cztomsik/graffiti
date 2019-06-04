import * as React from 'react'
import { useState } from 'react'
import { TextInputProps } from 'react-native'
import View from './View';
import { Text } from './Text';
import StyleSheet from '../Stylesheet';

const TextInput = (props: TextInputProps) => {
  const [active, setActive] = useState(false)
  const textInput = useTextValue(props.value, props.onChangeText)

  return (
    <View style={[styles.input, active && styles.active, props.style]} {...textInput} onFocus={() => setActive(true)} onBlur={() => setActive(false)}>
      <Text style={styles.text}>{props.value}</Text>
    </View>
  )
}

const styles = StyleSheet.create({
  input: {
    paddingVertical: 6,
    paddingHorizontal: 12,
    borderColor: '#cde',
    borderRadius: 4,
    borderWidth: 1
  },

  active: {
    borderColor: '#8bf',
    shadowSpread: 3,
    shadowRadius: 0,
    shadowColor: '#07f4'
  },

  text: {
    lineHeight: 24,
    color: '#556'
  }
})

export default TextInput

// this is very basic for now
export const useTextValue = (value, onChange) => {
  const onKeyPress = e => {
      if ( ! e.key) {
        return
      }

      onChange(value + e.key)
  }

  const onKeyDown = e => {
    if (e.code === 'Backspace') {
      // TODO: caret position
      onChange(value.slice(0, -1))
    }
  }

  return {
    value,
    onKeyDown,
    onKeyPress
  }
}
