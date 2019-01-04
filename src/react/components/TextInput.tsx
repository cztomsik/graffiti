import * as React from 'react'
import { useState, useRef, useContext } from 'react'
import { TextInputProps } from '../react-native-types'
import { ControlManagerContext, TouchableWithoutFeedback, Text, StyleSheet } from '..'
import { ResourceManager } from '../..'

// TODO: reconsider if layout should be coupled to Surface, drawing custom shapes (caret) would have been much easier

// empty text would collapse (consider minWidth)
const HOLDER = ' '

const TextInput = (props: TextInputProps) => {
  const textInput = useTextInput(props.value, props.onChangeText)

  return (
    <TouchableWithoutFeedback style={styles.input} onPress={textInput.focus}>
      <Text>{props.value || HOLDER}</Text>
    </TouchableWithoutFeedback>
  )
}

const styles = StyleSheet.create({
  input: {
    borderColor: '#cccccc',
    borderBottomWidth: 1
  }
})

export default TextInput


const useTextInput = (value, onChange) => {
  const controlManager = useContext(ControlManagerContext)
  const textInputRef = useRef(undefined)

  if (textInputRef.current === undefined) {
    textInputRef.current = createTextInput(controlManager)
  }

  const textInput = textInputRef.current

  textInput.value = value
  textInput.onChange = onChange

  return textInput
}

const createTextInput = (controlManager) => {
  const handler = {
    keyPress: (e) => {
      // backspace
      if (e.ch === '\u007f') {
        return textInput.onChange(textInput.value = textInput.value.slice(0, -1))
      }

      if ( ! e.ch) {
        return
      }

      textInput.onChange(textInput.value = textInput.value + e.ch)
    },
    focus: () => {},
    blur: () => {}
  }

  const textInput = {
    value: '',
    onChange: (str) => {},
    focus: () => controlManager.focus(handler),
    // TODO: only if we're focused
    blur: () => controlManager.blur()
  }

  return textInput
}
