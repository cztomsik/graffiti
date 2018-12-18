import * as React from 'react'
import { SwitchProps } from '../react-native-types'
import { View, Text, StyleSheet } from '..'

const Switch = (props: SwitchProps) =>
  <View style={[styles.track, props.disabled && styles.trackDisabled]}>
    <View style={[styles.thumb, props.value && styles.thumbOn]}></View>
  </View>

const styles = StyleSheet.create({
  track: {
    width: 100,
    backgroundColor: '#a3d3cf'
  },

  trackDisabled: {
    backgroundColor: '#d5d5d5'
  },

  thumb: {
    width: 40,
    backgroundColor: '#ffffff'
  },

  thumbOn: {
    marginLeft: 60
  }
})

export default Switch
