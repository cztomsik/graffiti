import * as React from 'react'
import { SwitchProps } from '../react-native-types'
import { View, Text, StyleSheet, TouchableWithoutFeedback } from '..'

const Switch = (props: SwitchProps) => {
  const { disabled, value, onValueChange } = props

  return (
    <TouchableWithoutFeedback onPress={onValueChange as any}>
      <View style={[styles.track, value && styles.trackActive, disabled && styles.trackDisabled]}>
        <View style={[styles.thumb, value && styles.thumbActive]} />
      </View>
    </TouchableWithoutFeedback>
  )
}

const styles = StyleSheet.create({
  // TODO: disabling might be done using opacity and/or color filter

  track: {
    width: 80,
    height: 28,
    padding: 3,

    backgroundColor: '#bdbdbd'
  },

  trackActive: {
    backgroundColor: '#2196f3'
  },

  trackDisabled: {
    backgroundColor: '#f5f5f5'
  },

  thumb: {
    height: '100%',
    width: 35,
    backgroundColor: '#ffffff'
  },

  thumbActive: {
    marginLeft: 39
  }
})

export default Switch
