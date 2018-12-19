import * as React from 'react'
import { ButtonProps } from '../react-native-types'
import { TouchableWithoutFeedback, View, Text, StyleSheet } from '..'

const Button = (props: ButtonProps) =>
  <TouchableWithoutFeedback onPress={props.onPress}>
    <View style={[styles.button, props.disabled && styles.buttonDisabled]}>
      <Text style={[styles.text, props.disabled && styles.textDisabled]}>
        {props.title.toUpperCase()}
      </Text>
    </View>
  </TouchableWithoutFeedback>

const styles = StyleSheet.create({
  button: {
    backgroundColor: '#2196F3',
    padding: 10,
    borderRadius: 2
  },

  text: {
    color: '#ffffff',
    textAlign: 'center'
  },

  buttonDisabled: {
    backgroundColor: '#dfdfdf'
  },

  textDisabled: {
    color: '#a1a1a1'
  }
})

export default Button
