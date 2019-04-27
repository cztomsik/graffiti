import * as React from 'react'
import { ButtonProps } from '../react-native-types'
import View from './View';
import { Text } from './Text';
import StyleSheet from '../Stylesheet';

const Button = (props: ButtonProps) =>
  <View style={[styles.button, props.disabled && styles.buttonDisabled]} onClick={props.onPress}>
    <Text style={[styles.text, props.disabled && styles.textDisabled]}>
      {props.title.toUpperCase()}
    </Text>
  </View>

const styles = StyleSheet.create({
  button: {
    justifyContent: 'space-around',
    backgroundColor: '#2196F3',
    paddingHorizontal: 10,
    borderRadius: 2
  },

  text: {
    fontSize: 14,
    lineHeight: 32,
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
