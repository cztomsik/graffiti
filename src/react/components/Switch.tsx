import * as React from 'react'
import { SwitchProps } from '../react-native-types'
import View from './View';
import { Text } from './Text';
import TouchableWithoutFeedback from './TouchableWithoutFeedback';
import StyleSheet from '../Stylesheet';

const Switch = (props: SwitchProps) => {
  const { disabled, value, onValueChange } = props

  return (
    <TouchableWithoutFeedback style={styles.ct} onPress={onValueChange as any}>
      <View style={[styles.track, value && styles.trackActive, disabled && styles.trackDisabled]}>
        <View style={[styles.thumb, value && styles.thumbActive]} />
        <Text style={styles.mark}>{value ?'ON' :'OFF'}</Text>
      </View>
    </TouchableWithoutFeedback>
  )
}

const styles = StyleSheet.create({
  // TODO: consider using opacity and/or color filter for disabled state
  ct: {
    alignSelf: 'center'
  },

  track: {
    flexDirection: 'row',
    width: 65,
    height: 25,
    padding: 3,
    borderRadius: 2,
    backgroundColor: '#bdbdbd'
  },

  trackActive: {
    flexDirection: 'row-reverse',
    backgroundColor: '#2196f3'
  },

  trackDisabled: {
    backgroundColor: '#f5f5f5'
  },

  thumb: {
    height: '100%',
    width: 22,
    borderRadius: 2,
    backgroundColor: '#ffffff'
  },

  thumbActive: {
  },

  mark: {
    alignSelf: 'center',
    marginLeft: 7,
    // TODO: measure is not called if width is set
    // width: 27,
    color: '#ffffff',
    fontSize: 12,
    lineHeight: 20
  }
})

export default Switch
